use nohash::IntMap as HashMap;
// use std::collections::HashMap;

use log::{debug, info, trace};
use rand::Rng;

use crate::{
    data_and_para::DataAndPara,
    tree::{node::Node, suff::Suff, tree::Tree},
};

pub trait TreeMutationProposer {
    fn tree(&mut self) -> &mut Tree;
    fn sigma(&self) -> f64;
    fn tau(&self) -> f64;
    fn resid(&self) -> &Vec<f64>;
    fn proposal<R: Rng + ?Sized>(&mut self, rng: &mut R) -> Option<Node>;
    fn gather_suff(&mut self, node: Option<Node>) -> HashMap<u8, Suff>;

    fn pr(&self, node: Option<Node>) -> f64;
    fn llh(&self, node: Option<Node>, all_suff: &HashMap<u8, Suff>) -> f64;

    fn do_accept_or_reject<R: Rng + ?Sized>(
        &mut self,
        node: Option<Node>,
        is_accept: bool,
        all_suff: &mut HashMap<u8, Suff>,
        rng: &mut R,
    );

    fn update_resid(&mut self, node: Option<Node>, all_suff: &HashMap<u8, Suff>);

    fn update_resid_not_feasible(tree: &Tree, resid: &mut Vec<f64>, all_suff: &HashMap<u8, Suff>) {
        tree.leaf_idx
            .iter()
            .zip(resid.iter_mut())
            .for_each(|(node_ptr, r)| {
                *r -= all_suff[&node_ptr].diff();
            });
    }

    fn update_resid_pg(
        tree: &mut Tree,
        node: Option<Node>,
        resid: &mut Vec<f64>,
        all_suff: &HashMap<u8, Suff>,
    ) {
        if node.is_none() {
            return Self::update_resid_not_feasible(tree, resid, all_suff);
        }

        let new = node.unwrap();
        //grow accept || prune reject
        if tree.is_nog(new.idx()) {
            return Self::update_resid_not_feasible(tree, resid, all_suff);
        }

        //grow reject || prune accept
        let (cur_idx, left_idx, right_idx) = (new.idx(), new.left_idx(), new.right_idx());
        tree.leaf_idx
            .iter_mut()
            .zip(resid.iter_mut())
            .for_each(|(node_ptr, r)| {
                *r -= all_suff[node_ptr].diff();
                if *node_ptr == left_idx || *node_ptr == right_idx {
                    *node_ptr = cur_idx;
                }
            });
    }

    fn sample_mu<R: Rng + ?Sized>(&mut self, all_suff: &mut HashMap<u8, Suff>, rng: &mut R) {
        self.tree().get_all_leaf_idx().iter().for_each(|&leaf_idx| {
            let suff = all_suff
                .get_mut(&leaf_idx)
                .expect("leaf_idx suff not exists");
            let new_mu = suff.sample_mu(self.sigma(), self.tau(), rng);
            suff.set_new_mu(new_mu);
            self.tree().set_mu(leaf_idx, new_mu);
        });
    }

    fn sample<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        info!("tree-bef:{}", self.tree());
        trace!("leaf-idx-1:{:?}", self.tree().leaf_idx);
        trace!("resid-bef:{:?}", self.resid());

        let node = self.proposal(rng);
        info!(
            "proposal node: {}",
            if node.is_none() {
                "none".to_string()
            } else {
                node.unwrap().to_string()
            }
        );

        let is_feasible = !node.is_none();
        let mut all_suff: HashMap<u8, Suff> = self.gather_suff(node);
        all_suff
            .iter_mut()
            .for_each(|(_, suff)| suff.add_old_mu_to_sy());
        debug!("gather_suff-1:{}", Suff::fmt_all_suff(&all_suff));
        trace!("leaf-idx-2:{:?}", self.tree().leaf_idx);

        let accept_prob: f64 = f64::ln(rng.gen_range(0.0..1.0));
        let (pr, llh) = (self.pr(node), self.llh(node, &all_suff));
        let lalpha = f64::min(pr + llh, 0.0);
        let is_accept = is_feasible && accept_prob < lalpha;
        info!(
            "is_accept:{}, is_feasible:{}, lalpha:{}, pr:{}, llh:{}, accept_prob:{}",
            is_feasible, lalpha, pr, llh, accept_prob, is_accept
        );
        self.do_accept_or_reject(node, is_accept, &mut all_suff, rng);
        debug!("gather_suff-2:{}", Suff::fmt_all_suff(&all_suff));

        trace!("resid-bef:{:?}", self.resid());
        self.update_resid(node, &all_suff);
        trace!("resid-aft:{:?}", self.resid());
        trace!("leaf-idx-3:{:?}", self.tree().leaf_idx);

        info!("tree-aft:{}", self.tree());
    }

    fn gather_suff_not_feasible(
        tree: &Tree,
        resid: &Vec<f64>,
        w: &Option<Vec<f64>>,
    ) -> HashMap<u8, Suff> {
        let mut ret: HashMap<u8, Suff> = tree
            .get_all_leaf_idx()
            .iter()
            .map(|&leaf_idx| {
                let mut suff = Suff::default();
                let old_mu = tree.mu(leaf_idx);
                suff.set_old_mu(old_mu);
                (leaf_idx, suff)
            })
            .collect();

        tree.leaf_idx
            .iter()
            .enumerate()
            .for_each(|(idx, leaf_idx)| ret.get_mut(leaf_idx).unwrap().update(idx, resid, w));

        ret
    }

    fn log_pr_pg(node: Option<Node>, tree: &Tree, data_para: &DataAndPara) -> f64 {
        if node.is_none() {
            return 0.0;
        }
        let new = node.unwrap();
        let is_grow = tree.is_leaf(new.idx());
        let (num_leaves, num_nogs) = match is_grow {
            true => {
                let num_leaves = tree.get_all_leaf_idx().len() as f64;
                let num_nogs = if tree.is_nog(new.idx()) {
                    tree.get_all_nog_idx().len() as f64
                } else {
                    (tree.get_all_nog_idx().len() as f64) + 1.0
                };
                (num_leaves, num_nogs)
            }
            false => {
                let num_leaves = (tree.get_all_leaf_idx().len() as f64) - 1.0;
                let num_nogs = tree.get_all_nog_idx().len() as f64;
                (num_leaves, num_nogs)
            }
        };
        let d = new.depth();

        debug!(
            "prune:{}, grow:{}, d:{}",
            data_para.prob_prune, data_para.prob_grow, d
        );
        debug!(
            "split(d + 1):{}, split(d):{}, not_split(d):{}",
            data_para.prob_not_split(d + 1),
            data_para.prob_split(d),
            data_para.prob_not_split(d)
        );
        debug!("num_leaves:{}, num_nogs:{}", num_leaves, num_nogs);
        f64::ln(data_para.prob_prune / data_para.prob_grow)
            + f64::ln(data_para.prob_not_split(d + 1)) * 2.0
            + f64::ln(data_para.prob_split(d))
            - f64::ln(data_para.prob_not_split(d))
            + f64::ln(num_leaves / num_nogs)
    }

    fn lr_diff(
        node: Option<Node>,
        all_suff: &HashMap<u8, Suff>,
        sigma: f64,
        tau: f64,
        n_eta: f64,
    ) -> f64 {
        if node.is_none() {
            return 0.0;
        }
        let new = node.unwrap();
        let left = all_suff[&new.left_idx()];
        let right = all_suff[&new.right_idx()];
        let parent = Suff::merge(&left, &right);
        debug!(
            "new:{}, left:{}, right:{}, parent:{},, sigma:{}, tau:{}",
            new,
            left.llh(sigma, tau, n_eta),
            right.llh(sigma, tau, n_eta),
            parent.llh(sigma, tau, n_eta),
            sigma,
            tau
        );
        left.llh(sigma, tau, n_eta) + right.llh(sigma, tau, n_eta) - parent.llh(sigma, tau, n_eta)
    }
}
