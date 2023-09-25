use nohash::IntMap as HashMap;
// use std::collections::HashMap;

use crate::{
    data_and_para::DataAndPara,
    tree::{node::Node, suff::Suff, tree::Tree},
};
use log::info;
use rand::{seq::SliceRandom, Rng};
use rayon::{
    prelude::{
        IndexedParallelIterator,
        ParallelIterator,
    },
    slice::ParallelSliceMut,
};

use super::tree_mutation_proposer::TreeMutationProposer;

#[derive(Debug)]
pub struct GrowProposer<'a> {
    pub tree: &'a mut Tree,
    pub resid: &'a mut Vec<f64>,
    pub data_para: &'a DataAndPara,
    pub sigma: f64,
    pub tau: f64,
}

impl<'a> TreeMutationProposer for GrowProposer<'a> {
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
        info!("start grow proposal");

        let all_leaf_idx = self.tree.get_all_leaf_idx();
        info!("candidate node idx:{:?}", all_leaf_idx);
        let num_leaves = all_leaf_idx.len();
        if num_leaves == 0 {
            info!("no splittable leaf node");
            return None;
        }
        let node_idx = *all_leaf_idx.choose(rng).unwrap();
        info!("picked node idx: {}", node_idx);
        if Node::get_idx_depth(node_idx) >= 7 {
            info!("already at depth 7, can't continue grow");
            return None;
        }

        let picked_feat = *self.data_para.init_splits.choose(rng).unwrap();
        let split = picked_feat.rand_pick_sub_split(self.tree, node_idx, rng);
        match split {
            Some(split) => Some(Node::internal(node_idx, split)),
            None => {
                info!("picked node is not splittable");
                None
            }
        }
    }

    fn gather_suff(&mut self, node: Option<Node>) -> HashMap<u8, Suff> {
        if node.is_none() {
            return Self::gather_suff_not_feasible(&self.tree, &self.resid, &self.data_para.w);
        }

        let new = node.unwrap();
        let (cur_idx, left_idx, right_idx) = (new.idx(), new.left_idx(), new.right_idx());
        let mut all_node_idx: Vec<u8> = self.tree.get_all_leaf_idx();
        all_node_idx.retain(|&x| x != cur_idx);
        all_node_idx.extend(vec![left_idx, right_idx]);

        let mut ret: HashMap<u8, Suff> = all_node_idx
            .iter()
            .map(|&leaf_idx| {
                let mut suff = Suff::default();
                let old_mu = if leaf_idx == left_idx || leaf_idx == right_idx {
                    self.tree.mu(Node::get_parent_idx(leaf_idx))
                } else {
                    self.tree.mu(leaf_idx)
                };
                suff.set_old_mu(old_mu);
                (leaf_idx, suff)
            })
            .collect();

        let n = self.resid.len() as f64;
        let chunk_size = f64::ceil(n / (num_cpus::get() as f64)) as usize;
        let ret_ret: Vec<HashMap<u8, Suff>> = self
            .tree
            .leaf_idx
            .par_chunks_mut(chunk_size)
            .enumerate()
            .map(|(chunk_idx, sub)| {
                let mut ret = ret.clone();

                sub.iter_mut().enumerate().for_each(|(idx, node_ptr)| {
                    let idx = idx + chunk_idx * chunk_size;
                    if *node_ptr == cur_idx && new.go_left(&self.data_para.x, idx) {
                        ret.get_mut(&left_idx)
                            .unwrap()
                            .update(idx, self.resid, &self.data_para.w);
                        *node_ptr = left_idx;
                    } else if *node_ptr == cur_idx {
                        ret.get_mut(&right_idx)
                            .unwrap()
                            .update(idx, self.resid, &self.data_para.w);
                        *node_ptr = right_idx;
                    } else {
                        ret.get_mut(&node_ptr)
                            .unwrap()
                            .update(idx, self.resid, &self.data_para.w)
                    }
                });
                ret
            })
            .collect();
        ret.iter_mut().for_each(|(leaf_idx, suff)| {
            ret_ret
                .iter()
                .for_each(|sub_ret| suff.merge_inplace(&sub_ret[&leaf_idx]))
        });
        ret
    }

    fn pr(&self, node: Option<Node>) -> f64 {
        Self::log_pr_pg(node, self.tree, self.data_para)
    }

    fn llh(&self, node: Option<Node>, all_suff: &HashMap<u8, Suff>) -> f64 {
        Self::lr_diff(node, all_suff, self.sigma, self.tau, self.data_para.n_eta)
    }

    fn do_accept_or_reject<R: Rng + ?Sized>(
        &mut self,
        node: Option<Node>,
        is_accept: bool,
        all_suff: &mut HashMap<u8, Suff>,
        rng: &mut R,
    ) {
        if node.is_none() {
            self.sample_mu(all_suff, rng);
            return;
        }

        let new = node.unwrap();
        if is_accept {
            self.tree.grow_or_change(new);
            self.sample_mu(all_suff, rng);
        } else {
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
    }

    fn update_resid(&mut self, node: Option<Node>, all_suff: &HashMap<u8, Suff>) {
        Self::update_resid_pg(&mut self.tree, node, self.resid, all_suff);
    }
}
