use super::node::Node;
use core::fmt;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Tree {
    nodes: HashMap<u8, Node>,
    leaf_mu: HashMap<u8, f64>,
    pub leaf_idx: Vec<u8>,
}

impl Tree {
    pub fn new(n: usize) -> Self {
        if n == 0 {
            panic!("all sample size is 0");
        }
        let leaf_idx = vec![1; n as usize];
        let root = Node::leaf(1);
        let leaf_mu = HashMap::from_iter([(root.idx(), 0.0)]);
        let nodes = HashMap::from_iter([(root.idx(), root)]);
        Tree {
            nodes,
            leaf_mu,
            leaf_idx,
        }
    }

    #[inline]
    pub fn get_node(&self, idx: u8) -> &Node {
        self.nodes.get(&idx).expect("node idx not exists")
    }

    #[inline]
    pub fn get_mut_node(&mut self, idx: u8) -> &mut Node {
        self.nodes.get_mut(&idx).expect("node idx not exists")
    }

    #[inline]
    pub fn mu(&self, idx: u8) -> f64 {
        *self.leaf_mu.get(&idx).expect("node idx not exists")
    }

    #[inline]
    pub fn set_mu(&mut self, idx: u8, mu: f64) -> f64 {
        let old_mu = self.mu(idx);
        *self.leaf_mu.get_mut(&idx).unwrap() = mu;
        old_mu
    }

    #[inline]
    fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.idx(), node);
    }

    #[inline]
    pub fn is_leaf(&self, node_idx: u8) -> bool {
        self.nodes.contains_key(&node_idx) && self.nodes[&node_idx].is_leaf()
    }

    #[inline]
    pub fn is_nog(&self, node_idx: u8) -> bool {
        let left_idx = Node::get_left_idx(node_idx);
        let right_idx = Node::get_right_idx(node_idx);
        self.nodes.contains_key(&node_idx) && self.is_leaf(left_idx) && self.is_leaf(right_idx)
    }

    #[inline]
    pub fn get_all_leaf_idx(&self) -> Vec<u8> {
        self.leaf_mu.keys().copied().collect()
    }

    #[inline]
    pub fn get_all_nog_idx(&self) -> Vec<u8> {
        let mut leaf_idxes = self.get_all_leaf_idx();
        leaf_idxes.sort();
        let mut ret: Vec<u8> = Vec::new();
        let mut i = 0;
        while i < leaf_idxes.len() - 1 {
            match (
                Node::idx_is_left(leaf_idxes[i]),
                leaf_idxes[i + 1] - leaf_idxes[i] == 1,
            ) {
                (true, true) => {
                    ret.push(Node::get_parent_idx(leaf_idxes[i]));
                    i += 2;
                }
                _ => {
                    i += 1;
                }
            }
        }
        ret
    }

    #[inline]
    pub fn grow_or_change(&mut self, new: Node) {
        debug_assert!(
            self.is_leaf(new.idx()) || self.is_nog(new.idx()),
            "tree:{:?}, curr node:{} only for leaf/singly internal node",
            self,
            new
        );
        let (left_idx, right_idx) = (new.left_idx(), new.right_idx());
        let left = Node::leaf(left_idx);
        let right = Node::leaf(right_idx);
        if self.is_leaf(new.idx()) {
            self.add_node(new);
            self.add_node(left);
            self.add_node(right);
            self.leaf_mu.remove(&new.idx());
            self.leaf_mu.insert(left_idx, 0.0);
            self.leaf_mu.insert(right_idx, 0.0);
        } else {
            self.add_node(new);
        }
    }

    #[inline]
    pub fn prune(&mut self, new: Node) {
        if !(self.is_nog(new.idx()) && new.is_leaf()) {
            panic!("prune only allow at singly internal node:\n{}", self);
        }
        self.add_node(new);
        self.nodes.remove(&new.left_idx());
        self.nodes.remove(&new.right_idx());

        self.leaf_mu.insert(new.idx(), 0.0);
        self.leaf_mu.remove(&new.left_idx());
        self.leaf_mu.remove(&new.right_idx());
    }

    #[inline]
    pub fn predict(&self, x: &Vec<Vec<i16>>, idx: usize) -> u8 {
        let mut node = self.get_node(1);
        loop {
            if node.is_leaf() {
                break node.idx();
            }
            let child_idx = node.child_idx(x, idx);
            node = self.get_node(child_idx);
        }
    }

    #[inline]
    pub fn predict_mu(&self, x: &Vec<Vec<i16>>, idx: usize) -> f64 {
        self.leaf_mu[&self.predict(x, idx)]
    }
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //get all leaf nodex idx in preodrder
        fn get_all_leaf_idx(tree: &Tree) -> Vec<u8> {
            let mut ret: Vec<u8> = Vec::new();
            let mut node_stack: Vec<u8> = vec![1];
            let mut idx = 1;
            while !node_stack.is_empty() {
                loop {
                    if tree.is_leaf(idx) {
                        ret.push(idx);
                        break;
                    }
                    node_stack.push(idx);
                    idx = Node::get_left_idx(idx);
                }
                if !node_stack.is_empty() {
                    idx = Node::get_right_idx(node_stack.pop().unwrap());
                }
            }
            ret
        }

        fn get_id_path(tree: &Tree, node_idx: u8) -> String {
            fn fmt(tree: &Tree, idx: u8, is_left: bool) -> String {
                let node = tree.get_node(idx);
                let node_str = if tree.is_leaf(idx) {
                    format!("{:3.2}", tree.leaf_mu[&idx])
                } else {
                    format!("{}", node.split())
                };
                let s = format!("{:>3}({})", idx, node_str);
                if is_left {
                    s
                } else {
                    s.replace("< ", ">=").replace("in", "not in")
                }
            }
            let mut ret: Vec<String> = Vec::new();
            ret.push(fmt(tree, node_idx, false));
            let mut parent_idx: u8 = node_idx;
            while parent_idx > 1 {
                let is_left = Node::idx_is_left(parent_idx);
                parent_idx = Node::get_parent_idx(parent_idx);
                ret.push(fmt(tree, parent_idx, is_left));
            }
            ret.reverse();
            ret.join(" ->")
        }

        // let fmt_tree = self.get_all_leaf_idx()
        let fmt_tree = get_all_leaf_idx(&self)
            .iter()
            .map(|&idx| get_id_path(self, idx))
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "\n{}", fmt_tree)
    }
}

#[cfg(test)]
mod tests {
    use crate::tree::split::Split;

    use super::*;

    #[test]
    fn test_leaf_idx() {
        let mut tree = Tree::new(10);

        println!(
            "init: \n{}, nnog:{}, nleaf:{}",
            tree,
            tree.get_all_nog_idx().len(),
            tree.get_all_leaf_idx().len()
        );
        println!("init: \n{}", tree.is_leaf(1));

        tree.grow_or_change(Node::internal(1, Split::new_continuous(0, 4)));
        println!("grow 1: \n{}", tree);
        tree.grow_or_change(Node::internal(2, Split::new_continuous(0, 4)));
        println!("grow 2: \n{}", tree);
        tree.grow_or_change(Node::internal(5, Split::new_continuous(0, 4)));
        println!("grow 5: \n{}", tree);

        tree.set_mu(10, 3.2);
        println!("mu: {}\n", tree.mu(10));

        let mut leaf_idx = tree.get_all_leaf_idx();
        leaf_idx.sort();
        assert_eq!(leaf_idx, vec![3, 4, 10, 11]);
        assert_eq!(tree.get_all_nog_idx(), vec![5]);

        tree.grow_or_change(Node::internal(10, Split::new_continuous(0, 4)));
        println!("grow 10: \n{}", tree);

        let mut leaf_idx = tree.get_all_leaf_idx();
        leaf_idx.sort();
        assert_eq!(leaf_idx, vec![3, 4, 11, 20, 21]);
        assert_eq!(tree.get_all_nog_idx(), vec![10]);

        tree.grow_or_change(Node::internal(3, Split::new_continuous(0, 4)));
        println!("grow 3: \n{}", tree);
        let mut leaf_idx = tree.get_all_leaf_idx();
        leaf_idx.sort();
        assert_eq!(leaf_idx, vec![4, 6, 7, 11, 20, 21]);
        assert_eq!(tree.get_all_nog_idx(), vec![3, 10]);

        tree.prune(Node::leaf(10));
        println!("prune 10: \n{}", tree);
        let mut leaf_idx = tree.get_all_leaf_idx();
        leaf_idx.sort();
        assert_eq!(leaf_idx, vec![4, 6, 7, 10, 11]);
        assert_eq!(tree.get_all_nog_idx(), vec![3, 5]);

        tree.grow_or_change(Node::internal(3, Split::new_continuous(0, 6)));
        println!("change 3: \n{}", tree);
        let mut leaf_idx = tree.get_all_leaf_idx();
        leaf_idx.sort();
        assert_eq!(leaf_idx, vec![4, 6, 7, 10, 11]);
        assert_eq!(tree.get_all_nog_idx(), vec![3, 5]);
    }
}
