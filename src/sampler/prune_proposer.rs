use nohash::IntMap as HashMap;
// use std::collections::HashMap;

use log::info;
use rand::{seq::SliceRandom, Rng};

use crate::{
    data_and_para::DataAndPara,
    tree::{node::Node, suff::Suff, tree::Tree},
};

use super::tree_mutation_proposer::TreeMutationProposer;

#[derive(Debug)]
pub struct PruneProposer<'a> {
    pub tree: &'a mut Tree,
    pub resid: &'a mut Vec<f64>,
    pub data_para: &'a DataAndPara,
    pub sigma: f64,
    pub tau: f64,
}

impl<'a> TreeMutationProposer for PruneProposer<'a> {
    fn tree(&mut self) -> &mut Tree {
        self.tree
    }

    fn sigma(&self) -> f64 {
        self.sigma
    }

    fn tau(&self) -> f64 {
        self.tau
    }

    fn resid(&self) -> &Vec<f64> {
        self.resid
    }

    fn proposal<R: Rng + ?Sized>(&mut self, rng: &mut R) -> Option<Node> {
        info!("start prune proposal");

        let all_nog_idx = self.tree.get_all_nog_idx();
        info!("candidate node idx:{:?}", all_nog_idx);
        let num_nogs = all_nog_idx.len();
        if num_nogs == 0 {
            info!("no nog node");
            return None;
        }
        let node_idx = *all_nog_idx.choose(rng).unwrap();
        info!("picked node idx: {}", node_idx);
        Some(self.tree().get_node(node_idx).as_leaf())
    }

    fn gather_suff(&mut self, _node: Option<Node>) -> HashMap<u8, Suff> {
        Self::gather_suff_not_feasible(&self.tree, &self.resid, &self.data_para.w)
    }

    fn pr(&self, node: Option<Node>) -> f64 {
        -Self::log_pr_pg(node, self.tree, self.data_para)
    }

    fn llh(&self, node: Option<Node>, all_suff: &HashMap<u8, Suff>) -> f64 {
        -Self::lr_diff(node, all_suff, self.sigma, self.tau, self.data_para.n_eta)
    }

    fn do_accept_or_reject<R: Rng + ?Sized>(
        &mut self,
        node: Option<Node>,
        is_accept: bool,
        all_suff: &mut HashMap<u8, Suff>,
        rng: &mut R,
    ) {
        if node.is_none() || !is_accept {
            self.sample_mu(all_suff, rng);
            return;
        }

        let new = node.unwrap();
        self.tree.prune(new);
        let cur_suff = Suff::merge(&all_suff[&new.left_idx()], &all_suff[&new.right_idx()]);
        all_suff.insert(new.idx(), cur_suff);

        self.sample_mu(all_suff, rng);
        let new_mu = self.tree.mu(new.idx());
        all_suff
            .get_mut(&new.left_idx())
            .unwrap()
            .set_new_mu(new_mu);
        all_suff
            .get_mut(&new.right_idx())
            .unwrap()
            .set_new_mu(new_mu);
    }

    fn update_resid(&mut self, node: Option<Node>, all_suff: &HashMap<u8, Suff>) {
        Self::update_resid_pg(&mut self.tree, node, self.resid, all_suff);
    }
}
