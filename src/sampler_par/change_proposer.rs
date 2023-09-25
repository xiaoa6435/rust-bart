use nohash::IntMap as HashMap;
// use std::collections::HashMap;

use crate::{ data_and_para::DataAndPara, tree::{ node::Node, suff::Suff, tree::Tree } };
use log::{ info, trace };
use rand::{ seq::SliceRandom, Rng };
use rayon::{
    prelude::{
        IndexedParallelIterator,
        IntoParallelRefMutIterator,
        ParallelIterator,
    },
    slice::ParallelSliceMut,
};

use super::tree_mutation_proposer::TreeMutationProposer;

#[derive(Debug)]
pub struct ChangeProposer<'a> {
    pub tree: &'a mut Tree,
    pub resid: &'a mut Vec<f64>,
    pub data_para: &'a DataAndPara,
    pub sigma: f64,
    pub tau: f64,
}

impl<'a> TreeMutationProposer for ChangeProposer<'a> {
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
        info!("start change proposal");

        let all_nog_idx = self.tree.get_all_nog_idx();
        info!("candidate node idx:{:?}", all_nog_idx);
        let num_nogs = all_nog_idx.len();
        if num_nogs == 0 {
            info!("no nog node");
            return None;
        }
        let node_idx = *all_nog_idx.choose(rng).unwrap();
        info!("picked node idx: {}", node_idx);
        if Node::get_idx_depth(node_idx) >= 6 {
            info!("already at depth 6, can't continue change");
            return None;
        }
        let old_split = self.tree.get_node(node_idx).split();
        let picked_feat = *self.data_para.init_splits.choose(rng).unwrap();
        let split = picked_feat.rand_pick_sub_split(self.tree, node_idx, rng);
        match split {
            Some(split) if split != old_split => Some(Node::internal(node_idx, split)),
            _ => {
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
        let (left_idx, right_idx) = (new.left_idx(), new.right_idx());
        let (left_right_idx, right_left_idx) = (
            Node::get_right_idx(left_idx),
            Node::get_left_idx(right_idx),
        );

        let mut all_node_idx = self.tree.get_all_leaf_idx();
        all_node_idx.extend(vec![left_right_idx, right_left_idx]);

        let mut ret: HashMap<u8, Suff> = all_node_idx
            .iter()
            .map(|&leaf_idx| {
                let mut suff = Suff::default();
                let old_mu = if leaf_idx == left_right_idx || leaf_idx == right_left_idx {
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
        let ret_ret: Vec<HashMap<u8, Suff>> = self.tree.leaf_idx
            .par_chunks_mut(chunk_size)
            .enumerate()
            .map(|(chunk_idx, sub)| {
                let mut ret = ret.clone();
                sub.iter_mut()
                    .enumerate()
                    .for_each(|(idx, node_ptr)| {
                        let idx = idx + chunk_idx * chunk_size;
                        if *node_ptr == left_idx && new.go_right(&self.data_para.x, idx) {
                            ret.get_mut(&left_right_idx)
                                .unwrap()
                                .update(idx, self.resid, &self.data_para.w);
                            // .update_no_w(idx, self.resid);
                            *node_ptr = left_right_idx;
                        } else if *node_ptr == right_idx && new.go_left(&self.data_para.x, idx) {
                            ret.get_mut(&right_left_idx)
                                .unwrap()
                                .update(idx, self.resid, &self.data_para.w);
                            // .update_no_w(idx, self.resid);
                            *node_ptr = right_left_idx;
                        } else {
                            ret.get_mut(&node_ptr)
                                .unwrap()
                                .update(idx, self.resid, &self.data_para.w);
                            // .update_no_w(idx, self.resid);
                        }
                    });
                ret
            })
            .collect();
        ret.iter_mut().for_each(|(leaf_idx, suff)| {
            ret_ret.iter().for_each(|sub_ret| suff.merge_inplace(&sub_ret[&leaf_idx]))
        });
        ret
    }

    fn pr(&self, _node: Option<Node>) -> f64 {
        0.0
    }

    fn llh(&self, node: Option<Node>, all_suff: &HashMap<u8, Suff>) -> f64 {
        if node.is_none() {
            return 0.0;
        }
        let new = node.unwrap();
        let (left_idx, right_idx) = (new.left_idx(), new.right_idx());
        let (left_right_idx, right_left_idx) = (
            Node::get_right_idx(left_idx),
            Node::get_left_idx(right_idx),
        );
        let old_left = Suff::merge(&all_suff[&left_idx], &all_suff[&left_right_idx]);
        let old_right = Suff::merge(&all_suff[&right_idx], &all_suff[&right_left_idx]);

        let new_left = Suff::merge(&all_suff[&left_idx], &all_suff[&right_left_idx]);
        let new_right = Suff::merge(&all_suff[&right_idx], &all_suff[&left_right_idx]);

        trace!(
            "new:{}, new_left:{}, new_right:{}, old_left:{}, old_right:{}, sigma:{}, tau:{}",
            new,
            new_left.llh(self.sigma, self.tau, self.data_para.n_eta),
            new_right.llh(self.sigma, self.tau, self.data_para.n_eta),
            old_left.llh(self.sigma, self.tau, self.data_para.n_eta),
            old_right.llh(self.sigma, self.tau, self.data_para.n_eta),
            self.sigma,
            self.tau
        );
        new_left.llh(self.sigma, self.tau, self.data_para.n_eta) +
            new_right.llh(self.sigma, self.tau, self.data_para.n_eta) -
            old_left.llh(self.sigma, self.tau, self.data_para.n_eta) -
            old_right.llh(self.sigma, self.tau, self.data_para.n_eta)
    }

    fn do_accept_or_reject<R: Rng + ?Sized>(
        &mut self,
        node: Option<Node>,
        is_accept: bool,
        all_suff: &mut HashMap<u8, Suff>,
        rng: &mut R
    ) {
        if node.is_none() {
            self.sample_mu(all_suff, rng);
            return;
        }

        let new = node.unwrap();
        let (left_idx, right_idx) = (new.left_idx(), new.right_idx());
        let (left_right_idx, right_left_idx) = (
            Node::get_right_idx(left_idx),
            Node::get_left_idx(right_idx),
        );

        let (l_idx, r_idx) = if is_accept {
            self.tree.grow_or_change(new);
            (right_left_idx, left_right_idx)
        } else {
            (left_right_idx, right_left_idx)
        };
        let new_left = Suff::merge(&all_suff[&left_idx], &all_suff[&l_idx]);
        let new_right = Suff::merge(&all_suff[&right_idx], &all_suff[&r_idx]);
        all_suff.insert(left_idx, new_left);
        all_suff.insert(right_idx, new_right);

        self.sample_mu(all_suff, rng);

        let left_new_mu = self.tree.mu(left_idx);
        let right_new_mu = self.tree.mu(right_idx);

        all_suff.get_mut(&l_idx).unwrap().set_new_mu(left_new_mu);
        all_suff.get_mut(&r_idx).unwrap().set_new_mu(right_new_mu);
    }

    fn update_resid(&mut self, node: Option<Node>, all_suff: &HashMap<u8, Suff>) {
        if node.is_none() {
            return Self::update_resid_not_feasible(self.tree, self.resid, all_suff);
        }

        let new = node.unwrap();
        let res = *self.tree.get_node(new.idx());
        let is_accept = res == new;
        let (left_idx, right_idx) = (new.left_idx(), new.right_idx());
        let (left_right_idx, right_left_idx) = (
            Node::get_right_idx(left_idx),
            Node::get_left_idx(right_idx),
        );

        self.tree.leaf_idx
            .par_iter_mut()
            .zip(self.resid.par_iter_mut())
            .for_each(|(node_ptr, r)| {
                *r -= all_suff[node_ptr].diff();
                if is_accept && *node_ptr == left_right_idx {
                    *node_ptr = right_idx;
                } else if is_accept && *node_ptr == right_left_idx {
                    *node_ptr = left_idx;
                } else if *node_ptr == left_right_idx {
                    *node_ptr = left_idx;
                } else if *node_ptr == right_left_idx {
                    *node_ptr = right_idx;
                }
            });
    }
}