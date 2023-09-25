// use std::collections::HashMap;
// use super::node::Node;

// #[derive(Clone, Debug)]
// pub struct LeafIdxIdx {
//     idx_cursor: HashMap<u32, (u32, u32)>,
//     idxidx: Vec<u32>,
// }

// impl LeafIdxIdx {
//     pub fn new(n: u32) -> Self {
//         if n == 0 {
//             panic!("n: {} > 0", n);
//         }
//         let idx_cursor = HashMap::from_iter([(0, (0, n))]);
//         let idxidx: Vec<u32> = (0..n).collect();
//         LeafIdxIdx { idx_cursor, idxidx }
//     }

//     pub fn reorder_idxidx(&mut self, x: &Vec<Vec<i16>>, node: &Node, left_n: u32) {
//         if left_n == 0 {
//             panic!("left_n: {} > 0", left_n);
//         }
//         let &(beg, end) = self.idx_cursor
//             .get(&node.idx())
//             .expect("self.idx_cursor always contains node idx");
//         let left_end = beg + left_n;
//         self.idx_cursor.insert(node.left_idx(), (beg, left_end));
//         self.idx_cursor.insert(node.right_idx(), (left_end, end));
//         let mut i = beg as usize;
//         let mut j = left_end as usize;
//         while i < (left_end as usize) && j < (end as usize) {
//             let xi = self.idxidx[i] as usize;
//             let xj = self.idxidx[j] as usize;
//             match (node.go_left(x, xi), node.go_right(x, xj)) {
//                 (true, true) => {
//                     i += 1;
//                     j += 1;
//                 }
//                 (false, false) => {
//                     self.idxidx.swap(i, j);
//                     i += 1;
//                     j += 1;
//                 }
//                 (true, false) => {
//                     i += 1;
//                 }
//                 (false, true) => {
//                     j += 1;
//                 }
//             }
//         }
//     }

//     pub fn get_idx_iter(&self, node_idx: u32) -> std::slice::Iter<u32> {
//         let (beg, end) = self.idx_cursor[&node_idx];
//         self.idxidx[beg as usize..end as usize].iter()
//     }
// }

// #[cfg(test)]
// mod tests {
//     use crate::tree::split::Split;

//     use super::*;

//     #[test]
//     fn test_split() {
//         let x: Vec<Vec<i16>> = vec![vec![1, 13, 2, 1, 0, 10]];
//         let n = x[0].len() as u32;
//         let mut leaf_idxidx = LeafIdxIdx::new(n);
//         //println!("{:?}", leaf_idxidx);
//         assert_eq!(leaf_idxidx.idxidx, vec![0, 1, 2, 3, 4, 5]);
//         let node = Node::internal(0, Split::new_continuous(0, 4));
//         let left_n: u32 = 4;
//         leaf_idxidx.reorder_idxidx(&x, &node, left_n);
//         //println!("{:?}", leaf_idxidx);
//         assert_eq!(leaf_idxidx.idxidx, vec![0, 4, 2, 3, 1, 5])
//     }
// }


    // #[inline]
    // pub fn mu(&self) -> f64 {
    //     if !self.is_leaf() {
    //         panic!("mu only for leaf node: {:?}", &self);
    //     }
    //     self.mu
    // }

    // #[inline]
    // pub fn set_mu(&mut self, mu: f64) -> f64 {
    //     if !self.is_leaf() {
    //         panic!("mu only for leaf node: {:?}", &self);
    //     }
    //     let old_mu = self.mu;
    //     self.mu = mu;
    //     old_mu
    // }


    //   #[test]
    //   fn test_no_hash() {

    //     use nohash_hasher::IntMap;

    //     let mut m: IntMap<u8, f64> = IntMap::default();
    //     println!("The useful size of `v` is {}", std::mem::size_of_val(&m));

    //     m.insert(0, 1.0);
    //     assert!(m.contains_key(&0));
    //     println!("The useful size of `v` is {}", std::mem::size_of_val(&m));

    //     m.insert(1, 2.0);
    //     assert!(m.contains_key(&1));
    //     m.insert(310, 3.0);
    //     assert!(m.contains_key(&310));
    //     println!("The useful size of `v` is {}", std::mem::size_of_val(&m));

    //     m.insert(510, 3.0);
    //     assert!(m.contains_key(&510));
    //     println!("The useful size of `v` is {}", std::mem::size_of_val(&m));

    //     m.insert(10240, 4.0);
    //     assert!(m.contains_key(&10240));
    //     println!("The useful size of `v` is {}", std::mem::size_of_val(&m));

    //   }


    // #[allow(dead_code)]
    // pub fn get_all_suff(
    //     tree: &mut Tree,
    //     new: Node,
    //     x: &Vec<Vec<i16>>,
    //     resid: &Vec<f64>,
    //     w: &Weight
    // ) -> HashMap<u8, Suff> {
    //     let mut ret: HashMap<u8, Suff> = HashMap::new();
    //     let parent_idx = new.idx();
    //     let (left_idx, right_idx) = (new.left_idx(), new.right_idx());
    //     let old = tree.get_node(new.idx()).clone();
    //     match (old.is_internal(), new.is_internal()) {
    //         (true, true) => {
    //             let (l_left_idx, r_left_idx) = (
    //                 Node::get_left_idx(right_idx),
    //                 Node::get_left_idx(left_idx),
    //             );
    //             tree.leaf_idx
    //                 .iter_mut()
    //                 .enumerate()
    //                 .for_each(|(idx, node_idx_ptr)| {
    //                     let node_idx = *node_idx_ptr;
    //                     Self::update_suff(node_idx, idx, resid, w, &mut ret);
    //                     if node_idx == left_idx && new.go_right(x, idx) {
    //                         Self::update_suff(l_left_idx, idx, resid, w, &mut ret);
    //                         *node_idx_ptr = l_left_idx;
    //                     } else if node_idx == right_idx && new.go_right(x, idx) {
    //                         *node_idx_ptr = r_left_idx;
    //                         Self::update_suff(r_left_idx, idx, resid, w, &mut ret);
    //                     }
    //                 });
    //         }
    //         (false, true) => {
    //             tree.leaf_idx
    //                 .iter_mut()
    //                 .enumerate()
    //                 .for_each(|(idx, node_idx_ptr)| {
    //                     let node_idx = *node_idx_ptr;
    //                     Self::update_suff(node_idx, idx, resid, w, &mut ret);
    //                     if node_idx == parent_idx {
    //                         if new.go_left(x, idx) {
    //                             Self::update_suff(left_idx, idx, resid, w, &mut ret);
    //                         } else {
    //                             Self::update_suff(right_idx, idx, resid, w, &mut ret);
    //                         }
    //                     }
    //                 });
    //         }
    //         (true, false) => {
    //             tree.leaf_idx
    //                 .iter_mut()
    //                 .enumerate()
    //                 .for_each(|(idx, node_idx_ptr)| {
    //                     let node_idx = *node_idx_ptr;
    //                     Self::update_suff(node_idx, idx, resid, w, &mut ret);
    //                 });
    //         }
    //         _ => panic!(),
    //     }
    //     ret
    // }

    // // #[allow(dead_code)]
    // // pub fn update_resid(
    // //     tree: &mut Tree,
    // //     new: Node,
    // //     resid: &mut Vec<f64>,
    // //     suff: HashMap<u8, f64>
    // //     // x: &Vec<Vec<i16>>,
    // //     // w: &Weight
    // // ) -> HashMap<u8, Suff> {
    // //     // let mut ret: HashMap<u8, Suff> = HashMap::new();
    // //     let old = tree.get_node(new.idx()).clone();
    // //     let is_accept = old == new;

    // //     let n: usize = resid.len();
    // //     let parent_idx = new.idx();
    // //     let (left_idx, right_idx) = (new.left_idx(), new.right_idx());

    // //     match (&mut tree.leaf_idx, old.is_internal(), new.is_internal()) {
    // //         (Some(ref mut leaf_idx), true, true) => {
    // //             let (l_left_idx, r_left_idx) = (
    // //                 Node::get_left_idx(right_idx),
    // //                 Node::get_left_idx(left_idx),
    // //             );
    // //             leaf_idx
    // //                 .iter_mut()
    // //                 .zip(resid.iter_mut())
    // //                 .for_each(|(node_idx_ptr, r)| {
    // //                     let node_idx = *node_idx_ptr;
    // //                     let d = suff[&node_idx];
    // //                     *r += suff[&node_idx];
    // //                     fun_name(is_accept, node_idx, l_left_idx, node_idx_ptr, right_idx, r_left_idx, left_idx);
    // //                 });
    // //         }
    // //         (Some(ref mut leaf_idx), false, true) => {
    // //             leaf_idx
    // //                 .iter_mut()
    // //                 .zip(resid.iter_mut())
    // //                 .for_each(|(node_idx_ptr, r)| {
    // //                     let node_idx = *node_idx_ptr;
    // //                     let d = suff[&node_idx];
    // //                     *r += suff[&node_idx];
    // //                     if !is_accept && (node_idx == left_idx && node_idx == right_idx) {
    // //                         *node_idx_ptr = parent_idx;
    // //                     } 
    // //                 });
    // //         }
    // //         (Some(ref mut leaf_idx), true, false) => {
    // //             leaf_idx
    // //                 .iter_mut()
    // //                 .zip(resid.iter_mut())
    // //                 .for_each(|(node_idx_ptr, r)| {
    // //                     let node_idx = *node_idx_ptr;
    // //                     let d = suff[&node_idx];
    // //                     *r += suff[&node_idx];
    // //                     if is_accept && (node_idx == left_idx && node_idx == right_idx) {
    // //                         *node_idx_ptr = parent_idx;
    // //                     } 
    // //                 });
    // //         }
    // //         // (Some(ref mut leaf_idx), false, true) => {
    // //         //     let (l_left_idx, r_left_idx) = (
    // //         //         Node::get_left_idx(right_idx),
    // //         //         Node::get_left_idx(left_idx),
    // //         //     );
    // //         //     leaf_idx
    // //         //         .iter_mut()
    // //         //         .zip(resid.iter_mut())
    // //         //         .for_each(|(node_idx_ptr, r)| {
    // //         //             let node_idx = *node_idx_ptr;
    // //         //             let d = suff[&node_idx];
    // //         //             *r += suff[&node_idx];
    // //         //             if is_accept && node_idx == l_left_idx {
    // //         //                 *node_idx_ptr = right_idx;
    // //         //             } else if is_accept && node_idx == r_left_idx {
    // //         //                 *node_idx_ptr = leaf_idx;
    // //         //             } else if !is_accept && node_idx == l_left_idx {
    // //         //                 *node_idx_ptr = leaf_idx;
    // //         //             } else if !is_accept && node_idx == r_left_idx {
    // //         //                 *node_idx_ptr = leaf_idx;
    // //         //             }
    // //         //         });
    // //         // }
    // //         // (Some(ref mut leaf_idx), false, true) => {
    // //         //     leaf_idx
    // //         //         .iter_mut()
    // //         //         .enumerate()
    // //         //         .for_each(|(idx, node_idx_ptr)| {
    // //         //             let node_idx = *node_idx_ptr;
    // //         //             Self::update_suff(node_idx, idx, resid, w, &mut ret);
    // //         //             if node_idx == parent_idx {
    // //         //                 if new.go_left(x, idx) {
    // //         //                     Self::update_suff(left_idx, idx, resid, w, &mut ret);
    // //         //                 } else {
    // //         //                     Self::update_suff(right_idx, idx, resid, w, &mut ret);
    // //         //                 }
    // //         //             }
    // //         //         });
    // //         // }
    // //         // (Some(ref mut leaf_idx), true, false) => {
    // //         //     leaf_idx
    // //         //         .iter_mut()
    // //         //         .enumerate()
    // //         //         .for_each(|(idx, node_idx_ptr)| {
    // //         //             let node_idx = *node_idx_ptr;
    // //         //             Self::update_suff(node_idx, idx, resid, w, &mut ret);
    // //         //         });
    // //         // }
    // //         _ => panic!(),
    // //     }
    // //     ret
    // // }

    // fn update_suff(
    //     node_idx: u8,
    //     idx: usize,
    //     resid: &Vec<f64>,
    //     w: &Weight,
    //     ret: &mut HashMap<u8, Suff>
    // ) {
    //     ret.entry(node_idx)
    //         .and_modify(|s| s.update(idx as usize, resid, w))
    //         .or_insert(Suff::default());
    // }


// fn fun_name(is_accept: bool, node_idx: u8, l_left_idx: u8, node_idx_ptr: &mut u8, right_idx: u8, r_left_idx: u8, left_idx: u8) {
//     if is_accept && node_idx == l_left_idx {
//         *node_idx_ptr = right_idx;
//     } else if is_accept && node_idx == r_left_idx {
//         *node_idx_ptr = left_idx;
//     } else if !is_accept && node_idx == l_left_idx {
//         *node_idx_ptr = left_idx;
//     } else if !is_accept && node_idx == r_left_idx {
//         *node_idx_ptr = right_idx;
//     }
// }


        // match (&mut tree.leaf_idx, old.is_internal(), new.is_internal()) {
        //     (None, true, true) => {
        //         let (l_left_idx, r_left_idx) = (
        //             Node::get_left_idx(left_idx),
        //             Node::get_left_idx(right_idx),
        //         );
        //         (0..n).for_each(|idx| {
        //             let node_idx = tree.predict_leaf_idx(x, idx) as u8;
        //             Self::update_suff(node_idx, idx, resid, w, &mut ret);
        //             if node_idx == left_idx && new.go_right(x, idx) {
        //                 Self::update_suff(l_left_idx, idx, resid, w, &mut ret);
        //             } else if node_idx == right_idx && new.go_right(x, idx) {
        //                 Self::update_suff(r_left_idx, idx, resid, w, &mut ret);
        //             }
        //         });
        //     }
        //     (None, false, true) => {
        //         (0..n).for_each(|idx| {
        //             let node_idx = tree.predict_leaf_idx(x, idx) as u8;
        //             Self::update_suff(node_idx, idx, resid, w, &mut ret);
        //             if node_idx == parent_idx && new.go_left(x, idx) {
        //                 if new.go_left(x, idx) {
        //                     Self::update_suff(left_idx, idx, resid, w, &mut ret);
        //                 } else {
        //                     Self::update_suff(right_idx, idx, resid, w, &mut ret);
        //                 }
        //             }
        //         });
        //     }
        //     (None, true, false) => {
        //         (0..n).for_each(|idx| {
        //             let node_idx = tree.predict_leaf_idx(x, idx) as u8;
        //             Self::update_suff(node_idx, idx, resid, w, &mut ret);
        //         });
        //     }
// fn update_suff_new(
//     node_idx: u8,
//     idx: usize,
//     new: Node,
//     x: &Vec<Vec<i16>>,
//     resid: &Vec<f64>,
//     w: &Weight,
//     ret: &mut HashMap<u8, Suff>,
//     mut new_suff: [Suff; 2],
//     left_idxidx: &mut Vec<usize>,
//     right_idxidx: &mut Vec<usize>
// ) {
//     Self::update_suff(node_idx, idx, resid, w, ret);
//     if node_idx == (new.idx() as u8) {
//         if new.go_left(x, idx) {
//             new_suff[0].update(idx, resid, &w);
//             left_idxidx.push(idx)
//         } else {
//             new_suff[1].update(idx, resid, &w);
//             right_idxidx.push(idx)
//         }
//     }
// }
// #[allow(dead_code)]
// fn getsuff(
//     tree: &Tree,
//     node_idx: u32,
//     split: Split,
//     x: &Vec<Vec<i16>>,
//     resid: Vec<f64>,
//     w: &Weight
// ) -> (Suff, Suff, Suff) {
//     if !(tree.is_leaf(node_idx) || tree.is_nog(node_idx)) {
//         panic!("node idx:{} is not singly internal or leaf node:\n{}", node_idx, &tree);
//     }

//     let mut suff_l = Suff::default();
//     let mut suff_r = Suff::default();
//     match tree.leaf_idxidx {
//         Some(ref leaf_idxidx) => {
//             leaf_idxidx
//                 .get_idx_iter(node_idx)
//                 .for_each(|&idx| {
//                     Self::update_suff(idx, split, &mut suff_l, &mut suff_r, x, &resid, w)
//                 });
//         }
//         None => {
//             let n = x[0].len() as u32;
//             (0..n)
//                 .filter(|&idx| { tree.is_belong_node_idx(node_idx, x, idx as usize) })
//                 .for_each(|idx| {
//                     Self::update_suff(idx, split, &mut suff_l, &mut suff_r, x, &resid, w)
//                 });
//         }
//     }
//     let suff = Suff::merge(&suff_l, &suff_r);
//     (suff_l, suff_r, suff)
// }

// #[allow(dead_code)]
// fn get_all_suff(
//     tree: &Tree,
//     // tgt_node_idx: u32,
//     x: &Vec<Vec<i16>>,
//     resid: Vec<f64>,
//     w: &Weight
// ) -> Vec<Suff> {
//     let leaf_idx_to_id: HashMap<u32, usize> = tree
//         .get_all_leaf_idx()
//         .iter()
//         .enumerate()
//         .map(|(id, &idx)| (idx, id))
//         // .filter(|&(idx, _id)| {
//         //     (idx != tgt_node_idx) &&
//         //         (idx != Node::get_left_idx(tgt_node_idx)) &&
//         //         (idx != Node::get_right_idx(tgt_node_idx))
//         // })
//         .collect();

//     let mut ret: Vec<Suff> = Vec::with_capacity(leaf_idx_to_id.len());
//     match tree.leaf_idxidx {
//         Some(ref leaf_idxidx) => {
//             leaf_idx_to_id
//                 .iter()
//                 .for_each(|(&node_idx, &id)| {
//                     leaf_idxidx
//                         .get_idx_iter(node_idx)
//                         .for_each(|&idx| { ret[id].update(idx as usize, &resid, w) })
//                 });
//         }
//         None => {
//             let n: usize = x[0].len();
//             (0..n).for_each(|idx| {
//                 let node_idx = tree.predict_leaf_idx(x, idx);
//                 let id = leaf_idx_to_id[&node_idx];
//                 ret[id].update(idx as usize, &resid, w)
//             });
//         }
//     }
//     ret
// }
// #[inline]
// fn update_suff(
//     idx: u32,
//     split: Split,
//     suff_l: &mut Suff,
//     suff_r: &mut Suff,
//     x: &Vec<Vec<i16>>,
//     resid: &Vec<f64>,
//     w: &Weight
// ) {
//     let idx = idx as usize;
//     if split.go_left(x, idx) {
//         suff_l.update(idx, &resid, &w)
//     } else {
//         suff_r.update(idx, &resid, &w)
//     }
// }


// #[inline]
//     pub fn is_belong_node_idx(&self, node_idx: u8, x: &Vec<Vec<i16>>, idx: usize) -> bool {
//         let mut node = self.get_node(0);
//         loop {
//             if node.idx() == node_idx {
//                 break true;
//             }
//             let child_idx = node.child_idx(x, idx);
//             node = self.get_node(child_idx);
//         }
//     }
// pub fn get_all_leaf_idx(&self) -> Vec<u8> {
// let mut ret: Vec<u8> = Vec::new();
// let mut node_stack: Vec<u8> = vec![1];
// let mut idx: u8 = 1;
// while !node_stack.is_empty() {
//     loop {
//         if self.is_leaf(idx) {
//             ret.push(idx);
//             break;
//         }
//         node_stack.push(idx);
//         idx = Node::get_left_idx(idx);
//     }
//     if !node_stack.is_empty() {
//         idx = Node::get_right_idx(node_stack.pop().unwrap());
//     }
// }
// ret
// }

// #[inline]
//     pub fn grow(&mut self, new: Node) {
//         if !self.is_leaf(new.idx()) {
//             panic!("grow only allow at leaf node:\n{}", self);
//         }
//         self.grow_or_change(new)
//     }

//     #[inline]
//     pub fn change(&mut self, new: Node) {
//         if !self.is_nog(new.idx()) {
//             panic!("change only allow at singly internal node:\n{}", self);
//         }
//         if new.split() == self.get_node(new.idx()).split() {
//             panic!("new split equal old split node:\n{}", self);
//         }
//         self.grow_or_change(new)
//     }
// #[inline]
// pub fn is_splittable(&self, node_idx: u8, init_split: Vec<Split>) -> bool {
//     //如果特征数量大于已经用到的split，那一定是可分的：这里假设每个原始变量都是可分的
//     if init_split.len() > (Node::get_idx_depth(node_idx) as usize) - 2 {
//         return true;
//     }
//     let mut parent_idx: u8 = node_idx;
//     while parent_idx > 1 {
//         parent_idx = Node::get_parent_idx(parent_idx);
//         if self.get_node(parent_idx).split().is_splittable(self, node_idx) {
//             return true;
//         }
//     }
//     false
// }

// #[inline]
// pub fn rand_pick_sub_split<R: Rng + ?Sized>(
//     &self,
//     node_idx: u32,
//     init_split: Split,
//     rng: &mut R
// ) -> Split {
//     init_split.rand_pick_sub_split(self, node_idx, rng).expect("node is splittable")
// }

// #[inline]
// pub fn rand_pick_sub_split<R: Rng + ?Sized>(
//     &self,
//     node_idx: u32,
//     init_split: &Vex<Split>,
//     rng: &mut R
// ) -> Split {
//     if init_split.len() > Node::get_idx_depth(node_idx) + 2 {

//     }
//     init_split.rand_pick_sub_split(self, node_idx, rng).expect("node is splittable")
// }

// #[derive(Clone, Debug)]
// pub struct Tree {
//     nodes: HashMap<u8, Node>,
//     pub leaf_idxidx: Option<LeafIdxIdx>,
// }

// impl Tree {
//     pub fn new(root_value: f64, n: u8) -> Self {
//         let leaf_idxidx = if n == 0 { None } else { Some(LeafIdxIdx::new(n)) };
//         let root = Node::leaf(1, root_value);
//         let nodes = HashMap::from_iter([(root.idx(), root)]);
//         Tree { nodes, leaf_idxidx }
//     }

//     #[inline]
//     pub fn get_node(&self, idx: u32) -> &Node {
//         self.nodes.get(&idx).expect("node idx not exists")
//     }

//     #[inline]
//     fn add_node(&mut self, node: Node) {
//         self.nodes.insert(node.idx(), node);
//     }

//     #[inline]
//     pub fn is_leaf(&self, idx: u32) -> bool {
//         self.nodes.contains_key(&idx) && self.nodes[&idx].is_leaf()
//     }

//     #[inline]
//     pub fn is_nog(&self, node_idx: u32) -> bool {
//         let left_idx = Node::get_left_idx(node_idx);
//         let right_idx = Node::get_right_idx(node_idx);
//         self.nodes.contains_key(&node_idx) && self.is_leaf(left_idx) && self.is_leaf(right_idx)
//     }

//     #[inline]
//     //get all leaf nodex idx in preodrder
//     pub fn get_all_leaf_idx(&self) -> Vec<u32> {
//         let mut ret: Vec<u32> = Vec::new();
//         let mut node_stack: Vec<u32> = vec![1];
//         let mut idx: u32 = 1;
//         while !node_stack.is_empty() {
//             loop {
//                 if self.is_leaf(idx) {
//                     ret.push(idx);
//                     break;
//                 }
//                 node_stack.push(idx);
//                 idx = Node::get_left_idx(idx);
//             }
//             if !node_stack.is_empty() {
//                 idx = Node::get_right_idx(node_stack.pop().unwrap());
//             }
//         }
//         ret
//     }

//     #[inline]
//     //get all internal nodex idx in preodrder
//     pub fn get_all_nog_idx(leaf_idxes: &Vec<u32>) -> Vec<u32> {
//         let mut ret: Vec<u32> = Vec::new();
//         let mut i = 0;
//         while i < leaf_idxes.len() - 1 {
//             match (Node::idx_is_left(leaf_idxes[i]), leaf_idxes[i + 1] - leaf_idxes[i] == 1) {
//                 (true, true) => {
//                     ret.push(Node::get_parent_idx(leaf_idxes[i]));
//                     i += 2;
//                 }
//                 _ => {
//                     i += 1;
//                 }
//             }
//         }
//         ret
//     }

//     #[inline]
//     fn grow_or_change(
//         &mut self,
//         node_idx: u32,
//         split: Split,
//         left_mu: f64,
//         right_mu: f64,
//         left_n: Option<u32>,
//         x: Option<&Vec<Vec<i16>>>
//     ) {
//         let new = Node::internal(node_idx, split);
//         let left = Node::leaf(new.left_idx(), left_mu);
//         let right = Node::leaf(new.right_idx(), right_mu);
//         match self.leaf_idxidx {
//             Some(ref mut left_idxidx) => {
//                 if x.is_none() || left_n.is_none() {
//                     panic!("leaf_idxidx is not none, while x or left_n is none");
//                 }
//                 left_idxidx.reorder_idxidx(x.unwrap(), &new, left_n.unwrap());
//             }
//             _ => {}
//         }
//         self.add_node(new);
//         self.add_node(left);
//         self.add_node(right);
//     }

//     #[inline]
//     pub fn grow(
//         &mut self,
//         node_idx: u32,
//         split: Split,
//         left_mu: f64,
//         right_mu: f64,
//         left_n: Option<u32>,
//         x: Option<&Vec<Vec<i16>>>
//     ) {
//         if !self.is_leaf(node_idx) {
//             panic!("grow only allow at leaf node:\n{}", self);
//         }
//         self.grow_or_change(node_idx, split, left_mu, right_mu, left_n, x)
//     }

//     #[inline]
//     pub fn change(
//         &mut self,
//         node_idx: u32,
//         split: Split,
//         left_mu: f64,
//         right_mu: f64,
//         left_n: Option<u32>,
//         x: Option<&Vec<Vec<i16>>>
//     ) {
//         if !self.is_nog(node_idx) {
//             panic!("change only allow at singly internal node:\n{}", self);
//         }
//         if split == self.get_node(node_idx).split() {
//             panic!("new split equal old split node:\n{}", self);
//         }
//         self.grow_or_change(node_idx, split, left_mu, right_mu, left_n, x)
//     }

//     #[inline]
//     pub fn prune(&mut self, node_idx: u32, mu: f64) {
//         if !self.is_nog(node_idx) {
//             panic!("prune only allow at singly internal node:\n{}", self);
//         }
//         let new = Node::leaf(node_idx, mu);
//         self.nodes.remove(&new.left_idx());
//         self.nodes.remove(&new.right_idx());
//         self.add_node(new);
//     }

//     #[inline]
//     pub fn is_belong_node_idx(&self, node_idx: u32, x: &Vec<Vec<i16>>, idx: usize) -> bool {
//         let mut node = self.get_node(0);
//         loop {
//             if node.idx() == node_idx {
//                 break true;
//             }
//             let child_idx = node.child_idx(x, idx);
//             node = self.get_node(child_idx);
//         }
//     }

//     #[inline]
//     pub fn predict(&self, x: &Vec<Vec<i16>>, idx: usize) -> Node {
//         let mut node = self.get_node(0);
//         loop {
//             if node.is_leaf() {
//                 break node.clone();
//             }
//             let child_idx = node.child_idx(x, idx);
//             node = self.get_node(child_idx);
//         }
//     }

//     #[inline]
//     pub fn predict_mu(&self, x: &Vec<Vec<i16>>, idx: usize) -> f64 {
//         self.predict(x, idx).mu()
//     }

//     #[inline]
//     pub fn predict_leaf_idx(&self, x: &Vec<Vec<i16>>, idx: usize) -> u32 {
//         self.predict(x, idx).idx()
//     }

//     #[inline]
//     pub fn is_splittable(&self, node_idx: u32, init_split: Vec<Split>) -> bool {
//         //如果特征数量大于已经用到的split，那一定是可分的：这里假设每个原始变量都是可分的
//         if init_split.len() > (Node::get_idx_depth(node_idx) as usize) - 2 {
//             return true;
//         }
//         let mut parent_idx: u32 = node_idx;
//         while parent_idx > 1 {
//             parent_idx = Node::get_parent_idx(parent_idx);
//             if self.get_node(parent_idx).split().is_splittable(self, node_idx) {
//                 return true;
//             }
//         }
//         false
//     }

//     // #[inline]
//     // pub fn rand_pick_sub_split<R: Rng + ?Sized>(
//     //     &self,
//     //     node_idx: u32,
//     //     init_split: Split,
//     //     rng: &mut R
//     // ) -> Split {
//     //     init_split.rand_pick_sub_split(self, node_idx, rng).expect("node is splittable")
//     // }

//     // #[inline]
//     // pub fn rand_pick_sub_split<R: Rng + ?Sized>(
//     //     &self,
//     //     node_idx: u32,
//     //     init_split: &Vex<Split>,
//     //     rng: &mut R
//     // ) -> Split {
//     //     if init_split.len() > Node::get_idx_depth(node_idx) + 2 {

//     //     }
//     //     init_split.rand_pick_sub_split(self, node_idx, rng).expect("node is splittable")
//     // }
// }
// mod tests {
//     use super::*;

//     #[test]
//     fn test_leaf_idx() {
//         let mut tree = Tree::new(3.0, 0);

//         println!("init: \n{}", tree);
//         tree.grow(1, Split::new_continuous(0, 4), 2.0, 3.0, None, None);
//         println!("grow 0: \n{}", tree);
//         tree.grow(2, Split::new_continuous(0, 4), 2.0, 3.0, None, None);
//         println!("grow 0: \n{}", tree);
//         tree.grow(5, Split::new_continuous(0, 4), 2.0, 3.0, None, None);
//         println!("grow 0: \n{}", tree);

//         let leaf_idx = tree.get_all_leaf_idx();
//         assert_eq!(leaf_idx, vec![4, 10, 11, 3]);
//         assert_eq!(Tree::get_all_nog_idx(&leaf_idx), vec![5]);

//         tree.grow(10, Split::new_continuous(0, 4), 2.0, 3.0, None, None);
//         println!("grow 10: \n{}", tree);

//         let leaf_idx = tree.get_all_leaf_idx();
//         assert_eq!(leaf_idx, vec![4, 20, 21, 11, 3]);
//         assert_eq!(Tree::get_all_nog_idx(&leaf_idx), vec![10]);

//         tree.grow(3, Split::new_continuous(0, 4), 2.0, 3.0, None, None);
//         println!("grow 3: \n{}", tree);
//         let leaf_idx = tree.get_all_leaf_idx();
//         assert_eq!(leaf_idx, vec![4, 20, 21, 11, 6, 7]);
//         assert_eq!(Tree::get_all_nog_idx(&leaf_idx), vec![10, 3]);

//         tree.prune(10, 2.0);
//         println!("prune 10: \n{}", tree);
//         let leaf_idx = tree.get_all_leaf_idx();
//         assert_eq!(leaf_idx, vec![4, 10, 11, 6, 7]);
//         assert_eq!(Tree::get_all_nog_idx(&leaf_idx), vec![5, 3]);

//         tree.change(3, Split::new_continuous(0, 6), 2.0, 3.0, None, None);
//         println!("change 3: \n{}", tree);
//         let leaf_idx = tree.get_all_leaf_idx();
//         assert_eq!(leaf_idx, vec![4, 10, 11, 6, 7]);
//         assert_eq!(Tree::get_all_nog_idx(&leaf_idx), vec![5, 3]);

//         // assert_eq!(tree.get_all, vec![4, 10, 11, 3])
//     }
// }

// ## v2
// #[derive(Clone, Debug)]
// pub struct Tree {
//     nodes: HashMap<u8, Node>,
//     pub leaf_idx: Option<Vec<u8>>,
// }

// impl Tree {
//     pub fn new(root_value: f64, n: u8) -> Self {
//         let leaf_idx = if n == 0 { None } else { Some(vec![1; n as usize]) };
//         let root = Node::leaf(1, root_value);
//         let nodes = HashMap::from_iter([(root.idx(), root)]);
//         Tree { nodes, leaf_idx }
//     }

//     #[inline]
//     pub fn get_node(&self, idx: u8) -> &Node {
//         self.nodes.get(&idx).expect("node idx not exists")
//     }

//     #[inline]
//     fn add_node(&mut self, node: Node) {
//         self.nodes.insert(node.idx(), node);
//     }

//     #[inline]
//     pub fn is_leaf(&self, idx: u8) -> bool {
//         self.nodes.contains_key(&idx) && self.nodes[&idx].is_leaf()
//     }

//     #[inline]
//     pub fn is_nog(&self, node_idx: u8) -> bool {
//         let left_idx = Node::get_left_idx(node_idx);
//         let right_idx = Node::get_right_idx(node_idx);
//         self.nodes.contains_key(&node_idx) && self.is_leaf(left_idx) && self.is_leaf(right_idx)
//     }

//     #[inline]
//     //get all leaf nodex idx in preodrder
//     pub fn get_all_leaf_idx(&self) -> Vec<u8> {
//         let mut ret: Vec<u8> = Vec::new();
//         let mut node_stack: Vec<u8> = vec![1];
//         let mut idx: u8 = 1;
//         while !node_stack.is_empty() {
//             loop {
//                 if self.is_leaf(idx) {
//                     ret.push(idx);
//                     break;
//                 }
//                 node_stack.push(idx);
//                 idx = Node::get_left_idx(idx);
//             }
//             if !node_stack.is_empty() {
//                 idx = Node::get_right_idx(node_stack.pop().unwrap());
//             }
//         }
//         ret
//     }

//     #[inline]
//     //get all internal nodex idx in preodrder
//     pub fn get_all_nog_idx(leaf_idxes: &Vec<u8>) -> Vec<u8> {
//         let mut ret: Vec<u8> = Vec::new();
//         let mut i = 0;
//         while i < leaf_idxes.len() - 1 {
//             match (Node::idx_is_left(leaf_idxes[i]), leaf_idxes[i + 1] - leaf_idxes[i] == 1) {
//                 (true, true) => {
//                     ret.push(Node::get_parent_idx(leaf_idxes[i]));
//                     i += 2;
//                 }
//                 _ => {
//                     i += 1;
//                 }
//             }
//         }
//         ret
//     }

//     #[inline]
//     fn grow_or_change(&mut self, new: Node, lr_idx: Option<(&Vec<usize>, &Vec<usize>)>) {
//         if !new.is_internal() {
//             panic!("grow_or_change only for internal node")
//         }
//         let (left_idx, right_idx) = (new.left_idx() as u8, new.right_idx() as u8);
//         let left = Node::leaf(left_idx as u8, 0.0);
//         let right = Node::leaf(right_idx as u8, 0.0);
//         match self.leaf_idx {
//             Some(ref mut leaf_idx) => {
//                 let (left_idxidx, right_idxidx) = lr_idx.unwrap();
//                 left_idxidx.iter().for_each(|&idx| leaf_idx[idx] = left_idx);
//                 right_idxidx.iter().for_each(|&idx| leaf_idx[idx] = right_idx);
//             },
//             None => {},
//         }
//         self.add_node(new);
//         self.add_node(left);
//         self.add_node(right);
//     }

//     #[inline]
//     pub fn grow(&mut self, new: Node, lr_idx: Option<(&Vec<usize>, &Vec<usize>)>) {
//         if !self.is_leaf(new.idx()) {
//             panic!("grow only allow at leaf node:\n{}", self);
//         }
//         self.grow_or_change(new, lr_idx)
//     }

//     #[inline]
//     pub fn change(&mut self, new: Node, lr_idx: Option<(&Vec<usize>, &Vec<usize>)>) {
//         if !self.is_nog(new.idx()) {
//             panic!("change only allow at singly internal node:\n{}", self);
//         }
//         if new.split() == self.get_node(new.idx()).split() {
//             panic!("new split equal old split node:\n{}", self);
//         }
//         self.grow_or_change(new, lr_idx)
//     }

//     #[inline]
//     pub fn prune(&mut self, new: Node) {
//         if !(self.is_nog(new.idx()) && new.is_leaf()) {
//             panic!("prune only allow at singly internal node:\n{}", self);
//         }
//         // let new = Node::leaf(node_idx, mu);
//         self.nodes.remove(&new.left_idx());
//         self.nodes.remove(&new.right_idx());
//         self.add_node(new);
//     }

//     #[inline]
//     pub fn is_belong_node_idx(&self, node_idx: u8, x: &Vec<Vec<i16>>, idx: usize) -> bool {
//         let mut node = self.get_node(0);
//         loop {
//             if node.idx() == node_idx {
//                 break true;
//             }
//             let child_idx = node.child_idx(x, idx);
//             node = self.get_node(child_idx);
//         }
//     }

//     #[inline]
//     pub fn predict(&self, x: &Vec<Vec<i16>>, idx: usize) -> Node {
//         let mut node = self.get_node(0);
//         loop {
//             if node.is_leaf() {
//                 break node.clone();
//             }
//             let child_idx = node.child_idx(x, idx);
//             node = self.get_node(child_idx);
//         }
//     }

//     #[inline]
//     pub fn predict_mu(&self, x: &Vec<Vec<i16>>, idx: usize) -> f64 {
//         self.predict(x, idx).mu()
//     }

//     #[inline]
//     pub fn predict_leaf_idx(&self, x: &Vec<Vec<i16>>, idx: usize) -> u8 {
//         self.predict(x, idx).idx()
//     }

//     #[inline]
//     pub fn is_splittable(&self, node_idx: u8, init_split: Vec<Split>) -> bool {
//         //如果特征数量大于已经用到的split，那一定是可分的：这里假设每个原始变量都是可分的
//         if init_split.len() > (Node::get_idx_depth(node_idx) as usize) - 2 {
//             return true;
//         }
//         let mut parent_idx: u8 = node_idx;
//         while parent_idx > 1 {
//             parent_idx = Node::get_parent_idx(parent_idx);
//             if self.get_node(parent_idx).split().is_splittable(self, node_idx) {
//                 return true;
//             }
//         }
//         false
//     }

//     // #[inline]
//     // pub fn rand_pick_sub_split<R: Rng + ?Sized>(
//     //     &self,
//     //     node_idx: u32,
//     //     init_split: Split,
//     //     rng: &mut R
//     // ) -> Split {
//     //     init_split.rand_pick_sub_split(self, node_idx, rng).expect("node is splittable")
//     // }

//     // #[inline]
//     // pub fn rand_pick_sub_split<R: Rng + ?Sized>(
//     //     &self,
//     //     node_idx: u32,
//     //     init_split: &Vex<Split>,
//     //     rng: &mut R
//     // ) -> Split {
//     //     if init_split.len() > Node::get_idx_depth(node_idx) + 2 {

//     //     }
//     //     init_split.rand_pick_sub_split(self, node_idx, rng).expect("node is splittable")
//     // }
// }

//pub mod mersenne_twister_fast;
//use mersenne_twister_fast;

// pub mod mersenne_twister;
// pub mod sample_zs;
// pub mod sampler;
// pub mod split;
// pub mod tree;
// use crate::mersenne_twister::mersenne_twister::MersenneTwister;

// pub mod tree;

// fn main() {
//     // println!("Hello, world!");
//     //println!("{}, mt init: {:?}", mt.mti, (&mt.mt));
//     // (0..10).for_each(|i| {
//     //     println!("i:{}, v:{}", i, mt.rand());
//     // });
    
//     let mut mt = MersenneTwister::new(321_i64);
//     let v: Vec<f64> = (0..100).map(|_| mt.rand()).collect();
//     println!("Array({:?})", v);
// }
// 624, mt init: [0, 1, 1812433255, 1900727105, 1208447044, -1813563330, -252359758, 337614300, -1062413356, 1018809052]
// i:0, v:0.5488135008937808
// i:1, v:0.7151893652121089
// i:2, v:0.6027633824638369
// i:3, v:0.5448831893094143
// i:4, v:0.42365480398481625
// i:5, v:0.645894115930523
// i:6, v:0.43758720602891743
// i:7, v:0.8917730012046243
// i:8, v:0.963662762532477
// i:9, v:0.3834415149340795

/*
spark-shell --jars /Users/zhangzhenhao/Documents/soft/bartMachine/target/bartMachine-1.0-SNAPSHOT.jar

import OpenSourceExtensions.{MersenneTwisterFast, MersenneTwisterFastSimple}

val sn = List(0.8859479397783847, 0.07791235700248489, 0.9796461534260021, 0.247671460944361, 0.7528847193547141, 0.5266756310323627, 0.9075537591833472, 0.8840703021402703, 0.0892689669997706, 0.5173445989979978, 0.34362128552460725, 0.21229368921028924, 0.3606734411950838, 0.27077517718697475, 0.7616250141503508, 0.4780418963428512, 0.09899468128179445, 0.2753947738748117, 0.794427312749077, 0.5139703133974578, 0.4532948041698249, 0.25515125674615113, 0.11397660567653178, 0.8243130571780694, 0.31775350333903796, 0.15230702868397794, 0.21497958514224658, 0.9121103156597843, 0.04311515156741852, 0.37595241552070446, 0.31796557117558977, 0.3540330248358138, 0.9333575785657794, 0.3885452043561015, 0.895939440153068, 0.14550321805217448, 0.4903603008084404, 0.9233404096302368, 0.8013113053846422, 0.848371817577571, 0.6654459694073619, 0.14321914183880213, 0.11609391137279146, 0.07739594438155417, 0.3829119280582791, 0.1464298554147475, 0.4478573063659629, 0.35552735794756196, 0.433141927869796, 0.8008066347222138, 0.35500567246365744, 0.04775059397264936, 0.8495783957914103, 0.6234256839479522, 0.1415989373431623, 0.9970798174477125, 0.4305509165114163, 0.9226012255390319, 0.2304064717858071, 0.5168484838658026, 0.7424501196843173, 0.08755963459316451, 0.20380958138490968, 0.5563697022485592, 0.006942200520867359, 0.3566111557472422, 0.849716763670419, 0.09130985929786317, 0.13033078977278945, 0.8193288592085761, 0.7112623395989045, 0.5156400436919587, 0.328492259259354, 0.6187460516589242, 0.8284395666192566, 0.5189309466263804, 0.6514653597544007, 0.8469896489517594, 0.477965073728414, 0.7203824652136139, 0.7057224041494631, 0.7744647452593825, 0.1578230775912821, 0.1004352663504603, 0.6398379233714129, 0.12224157985065243, 0.5436429560403032, 0.42278257672450414, 0.7493805342799493, 0.013975901829380133, 0.8492572852299506, 0.7796744533624277, 0.23807523313570533, 0.8702404220509012, 0.44369750440688593, 0.18578927740390994, 0.9272663525274942, 0.14601040405452081, 0.5637828725341845, 0.9385975350189193)
val mt = new MersenneTwisterFast(321L)
val so = List.range(0, 100).map(i => mt.nextDouble(false, false))
sn == so

sn.zip(so).zipWithIndex.foreach{v =>
    val ((r1, r2), i) = v
    println(s"i:${i}, eq: ${r1 == r2}, v1:${r1}, v2:${r2}")
}


// Range(0, 1000).map(i => {
//     val r1 = mt.nextDouble(false, false)
//     val r2 = mts.nextDouble(false)  
//     r1 == r2
// }).min

Range(0, 10).foreach(i => {
    //println(s"i:${i}, v:${mt.nextDouble}")
    //println(s"i:${i}, s:${mt.mt.slice(0, 5).toList}, v:${mt.nextDouble(false, false)}")
    val r1 = mt.nextDouble(false, false)
    val r2 = mts.nextDouble(false)
    println(s"i:${i}, eq: ${r1 == r2}, v1:${r1}, v2:${r2}")
})

// pub trait Sampler {
//     // type Data;
//     type Output;
//     // fn update(&mut self, data: &Self::Data);
//     fn sample<R: Rng + ?Sized>(&mut self, rng: &mut R) -> Self::Output;
// }

*/



// #[deny(arithmetic_overflow)]
// pub mod mersenne_twister {
//     const N: usize = 624;
//     const M: usize = 397;
//     const MATRIX_A: i32 = -1727483681; //0x9908b0df
//     const UPPER_MASK: i32 = -2147483648; // most significant w-r bits 0x80000000
//     const LOWER_MASK: i32 = 0x7fffffff; // least significant r bits
//     const K_INIT_OPERAND: i32 = 1812433253;
//     const TEMPERING_MASK_B: i32 = -1658038656i32; // 0x9d2c5680
//     const TEMPERING_MASK_C: i32 = -272236544i32; // 0xefc60000
//     const MAG01: [i32; 2] = [0x0, MATRIX_A];

//     #[derive(Debug)]
//     pub struct MersenneTwister {
//         mt: [i32; N as usize],
//         mti: usize,
//     }

//     impl MersenneTwister {
//         pub fn new(seed: i64) -> Self {
//             let mut mt = [0; N];
//             mt[0] = (seed & 0xffffffff) as i32;
//             let mut mti: usize = 1;
//             while mti < N {
//                 let prev = mt[mti - 1];
//                 mt[mti] = K_INIT_OPERAND * (prev ^ ((prev as u32) >> 30) as i32) + (mti as i32);
//                 mti += 1;
//             }
//             MersenneTwister { mt, mti }
//         }

//         fn twister(&mut self) -> i32 {
            
//             // generate N words at one time
//             if self.mti >= N {
//                 let mut kk: usize = 0;
//                 let mut y: i32;
//                 while kk < N - M {
//                     y = (self.mt[kk] & UPPER_MASK) | (self.mt[kk + 1] & LOWER_MASK);
//                     let y_s = ((y as u32) >> 1) as i32;
//                     self.mt[kk] = self.mt[kk + M] ^ y_s ^ MAG01[(y & 0x1) as usize];
//                     kk += 1;
//                 }

//                 while kk < N - 1 {
//                     y = (self.mt[kk] & UPPER_MASK) | (self.mt[kk + 1] & LOWER_MASK);
//                     let y_s = ((y as u32) >> 1) as i32;
//                     self.mt[kk] = self.mt[kk + M - N] ^ y_s ^ MAG01[(y & 0x1) as usize];
//                     kk += 1;
//                 }

//                 y = (self.mt[N - 1] & UPPER_MASK) | (self.mt[0] & LOWER_MASK);
//                 let y_s = ((y as u32) >> 1) as i32;
//                 self.mt[N - 1] = self.mt[M - 1] ^ y_s ^ MAG01[(y & 0x1) as usize];
//                 self.mti = 0;
//             }

//             let mut y = self.mt[self.mti];
//             self.mti += 1;
//             y ^= ((y as u32) >> 11) as i32;
//             y ^= (y << 7) & TEMPERING_MASK_B;
//             y ^= (y << 15) & TEMPERING_MASK_C;
//             y ^= ((y as u32) >> 18) as i32;
//             y
//         }

//         fn next_double(&mut self) -> f64 {
//             let y = self.twister();
//             let z = self.twister();
//             let r = (((((y as u32) >> 6) as i64) << 27) + (((z as u32) >> 5) as i64)) as f64;
//             r / (((1 as i64) << 53) as f64)
//         }

//         pub fn rand(&mut self) -> f64 {
//             let mut d = self.next_double();
//             while d <= 0.0 || d >= 1.0 {
//                 d = self.next_double();
//             }
//             d
//         }
//     }
// }
// use std::collections::HashMap;

// use rand::Rng;
// use crate::{ tree::{ tree::Tree, node::Node, suff::Suff }, DataAndPara::DataAndPara };

// use super::sampler::{ TreeMutationProposer, Sampler };

// #[derive(Debug)]
// pub struct GrowProposer<'a> {
//     tree: &'a mut Tree,
//     resid: &'a mut Vec<f64>,
//     data_para: &'a DataAndPara,
//     node: Option<Node>,
//     suff: HashMap<u8, Suff>,
//     sigma: f64,
//     tau: f64,
// }

// impl<'a> GrowProposer<'a> {
//     pub fn default(
//         tree: &'a mut Tree,
//         resid: &'a mut Vec<f64>,
//         data_para: &'a DataAndPara,
//         sigma: f64,
//         tau: f64
//     ) -> Self {
//         Self {
//             tree,
//             resid,
//             data_para,
//             node: None,
//             suff: HashMap::new(),
//             sigma,
//             tau,
//         }
//     }
// }

// impl<'a> Sampler for GrowProposer<'a> {
//     type Data = ();

//     type Output = ();

//     fn update(&mut self, _data: &Self::Data) {}

//     // fn sample<R: Rng + ?Sized>(&mut self, rng: &mut R) {
//     //     let is_feasible = self.proposal(rng);
//     //     self.gather_suff();
//     //     let accept_prob = f64::ln(rng.gen_range(0.0..1.0));
//     //     let (pr, llh) = (self.pr(), self.llh());
//     //     let is_accept = if is_feasible && pr + llh >= accept_prob {
//     //         self.tree.grow(self.node.unwrap());
//     //         true
//     //     } else {
//     //         false
//     //     };
//     //     self.tree
//     //         .get_all_leaf_idx()
//     //         .iter()
//     //         .for_each(|&leaf_idx| {
//     //             let mut suff = self.suff.get_mut(&leaf_idx).expect("leaf_idx suff not exists");
//     //             let new_mu = suff.sample_mu(self.sigma, self.tau, rng);
//     //             self.tree.get_mut_node(leaf_idx).set_mu(new_mu);
//     //             suff.new_mu = new_mu;
//     //         });
//     //     self.update_resid(is_accept);
//     // }
// }

// impl<'a> TreeMutationProposer for GrowProposer<'a> {
//     // fn sample<R: Rng + ?Sized>(&mut self, rng: &mut R) {
//     //     let is_feasible = self.proposal(rng);
//     //     self.gather_suff();
//     //     let accept_prob = f64::ln(rng.gen_range(0.0..1.0));
//     //     let (pr, llh) = (self.pr(), self.llh());
//     //     let is_accept = if is_feasible && pr + llh >= accept_prob {
//     //         self.tree.grow(self.node.unwrap());
//     //         true
//     //     } else {
//     //         false
//     //     };
//     //     self.tree
//     //         .get_all_leaf_idx()
//     //         .iter()
//     //         .for_each(|&leaf_idx| {
//     //             let mut suff = self.suff.get_mut(&leaf_idx).expect("leaf_idx suff not exists");
//     //             let new_mu = suff.sample_mu(self.sigma, self.tau, rng);
//     //             self.tree.get_mut_node(leaf_idx).set_mu(new_mu);
//     //             suff.new_mu = new_mu;
//     //         });
//     //     self.update_resid(is_accept);
//     // }

//     fn proposal<R: Rng + ?Sized>(&mut self, rng: &mut R) -> bool {
//         let all_leaf_idx = self.tree.get_all_leaf_idx();
//         if all_leaf_idx.len() == 0 {
//             return false;
//         }
//         let node_idx = all_leaf_idx[rng.gen_range(0..all_leaf_idx.len())];
//         let p = self.data_para.init_splits.len();
//         let feat_id = rng.gen_range(0..p);
//         let picked_split = self.data_para.init_splits[feat_id];
//         let split = picked_split.rand_pick_sub_split(self.tree, node_idx, rng);
//         if split.is_none() {
//             false
//         } else {
//             self.node = Some(Node::internal(node_idx, split.unwrap()));
//             true
//         }
//     }

//     fn gather_suff(&mut self) {
//         let mut ret: HashMap<u8, Suff> = HashMap::new();
//         let (is_feasible, parent_idx, left_idx, right_idx, new) = match self.node {
//             Some(new) => {
//                 let parent_idx = new.idx();
//                 let (left_idx, right_idx) = (new.left_idx(), new.right_idx());
//                 (true, parent_idx, left_idx, right_idx, new)
//             }
//             None => { (false, 0, 0, 0, Node::leaf(0, 0.0)) }
//         };
//         self.tree.leaf_idx
//             .iter_mut()
//             .enumerate()
//             .for_each(|(idx, node_idx_ptr)| {
//                 let node_idx = *node_idx_ptr;
//                 Self::update_suff(node_idx, idx, self.resid, &self.data_para.w, &mut ret);
//                 if is_feasible && node_idx == parent_idx {
//                     if new.go_left(&self.data_para.x, idx) {
//                         Self::update_suff(left_idx, idx, self.resid, &self.data_para.w, &mut ret);
//                         *node_idx_ptr = left_idx;
//                     } else {
//                         Self::update_suff(right_idx, idx, self.resid, &self.data_para.w, &mut ret);
//                         *node_idx_ptr = right_idx;
//                     }
//                 }
//             });
//         ret.iter_mut().for_each(|(&leaf_idx, suff)| {
//             if leaf_idx == left_idx || leaf_idx == right_idx {
//                 suff.old_mu = self.tree.get_node(parent_idx).mu();
//             } else {
//                 suff.old_mu = self.tree.get_node(leaf_idx).mu();
//             }
//         });
//     }

//     fn update_resid(&mut self, is_accept: bool) {
//         let (is_reject, parent_idx, left_idx, right_idx) = match self.node {
//             Some(new) => {
//                 let parent_idx = new.idx();
//                 let (left_idx, right_idx) = (new.left_idx(), new.right_idx());
//                 (!is_accept, parent_idx, left_idx, right_idx)
//             }
//             None => (false, 0, 0, 0),
//         };
//         self.tree.leaf_idx
//             .iter_mut()
//             .zip(self.resid.iter_mut())
//             .for_each(|(node_idx_ptr, r)| {
//                 let node_idx = *node_idx_ptr;
//                 *r += self.suff[&node_idx].old_mu;
//                 if is_reject && node_idx == left_idx && node_idx == right_idx {
//                     *node_idx_ptr = parent_idx;
//                 }
//             });
//     }

//     fn pr(&self) -> f64 {
//         match self.node {
//             Some(_) => {
//                 let d = self.node.unwrap().depth();
//                 let num_leaf = self.tree.get_all_leaf_idx().len() as f64;
//                 let num_nog = self.tree.get_all_nog_idx().len() as f64;
//                 f64::ln(self.data_para.prob_prune / self.data_para.prob_grow) +
//                     f64::ln(self.data_para.prob_depth_not(d + 1)) * 2.0 -
//                     f64::ln(self.data_para.prob_depth_not(d)) +
//                     f64::ln(num_leaf / num_nog)
//             }
//             None => 0.0,
//         }
//     }

//     fn llh(&self) -> f64 {
//         match self.node {
//             Some(new) => {
//                 let left_idx = new.left_idx() as u8;
//                 let right_idx = new.right_idx() as u8;
//                 let left = self.suff[&left_idx];
//                 let right = self.suff[&right_idx];
//                 let parent = Suff::merge(&left, &right);
//                 left.llh(self.sigma, self.tau) +
//                     right.llh(self.sigma, self.tau) -
//                     parent.llh(self.sigma, self.tau)
//             }
//             None => 0.0,
//         }
//     }
// }

// // #[cfg(test)]
// // mod tests {
// //     use super::*;

// //     #[test]
// //     fn test_leaf_idx() {

// //         #[derive(Debug)]
// //         struct SRef<'a> {
// //             v: &'a  mut Vec<i32>
// //         }
// //         let mut v = vec![1, 3];
// //         let sref = SRef{v: &mut v};
// //         // println!("sref: {:?}", &sref);
// //         // v.push(10);
// //         println!("sref: {:?}", &sref);
// //         // assert_eq!(tree.get_all, vec![4, 10, 11, 3])
// //     }
// // }


// // let all_leaves = if is_accept {
// //     self.tree.grow(self.node.unwrap());
// //     self.tree.get_all_leaf_idx()
// // } else {
// //     self.tree.get_all_leaf_idx()
// // };
// // all_leaves
// // self.confirm_proposal_and_update_leaf_resp(is_accept, rng);
// // let all_leaves = if is_accept {
// //     self.tree.grow(self.node.unwrap());
// //     self.tree.get_all_leaf_idx()
// // } else {
// //     self.tree.get_all_leaf_idx()
// // };
// // all_leaves
// // self.confirm_proposal_and_update_leaf_resp(is_accept, rng);
// // fn confirm_proposal_and_update_leaf_resp<R: Rng + ?Sized>(
// //     &mut self,
// //     accepted: bool,
// //     rng: &mut R
// // ) {
// //     let all_leaves = if !self.node.is_none() && accepted {
// //         self.tree.grow(self.node.unwrap());
// //         self.tree.get_all_leaf_idx()
// //     } else {
// //         self.tree.get_all_leaf_idx()
// //     };
// //     all_leaves.iter().for_each(|&leaf_idx| {
// //         let mut suff = self.suff.get_mut(&leaf_idx).unwrap();
// //         let new_mu = suff.sample_mu(self.sigma, self.tau, rng);
// //         self.tree.get_mut_node(leaf_idx).set_mu(new_mu);
// //         suff.new_mu = new_mu;
// //     });
// // }
// //     fn proposal<R: Rng + ?Sized>(&mut self, rng: &mut R) {
// //         let all_leaf_idx = self.tree.get_all_leaf_idx();
// //         if all_leaf_idx.len() == 0 {
// //             return;
// //         }
// //         let node_idx = all_leaf_idx[rng.gen_range(0..(all_leaf_idx.len()))];
// //         let p = self.init_splits.len();
// //         let feat_id = rng.gen_range(0..p);
// //         let picked_split = self.init_splits[feat_id];
// //         let split = picked_split.rand_pick_sub_split(self.tree, node_idx, rng);
// //         if split.is_none() {
// //             return;
// //         }
// //         self.node = Some(Node::internal(node_idx, split.unwrap()));
// //     }

// //     fn get_all_suff(&mut self,) -> HashMap<u8, Suff> {
// //         let mut ret: HashMap<u8, Suff> = HashMap::new();
// //         let mut new_suff = [Suff::default(); 2];
// //         let mut left_idxidx: Vec<usize> = Vec::new();
// //         let mut right_idxidx: Vec<usize> = Vec::new();
// //         // self.t
// //         // self.tree.lea
// //         // match (&tree.leaf_idx, new.is_leaf()) {
// //         match self.tree.leaf_idx {
// //             Some(ref mut leaf_idx) => {

// //                 Self::update_suff(node_idx, idx, resid, w, ret);
// //                 if node_idx == (new.idx() as u8) {
// //                     if new.go_left(x, idx) {
// //                         new_suff[0].update(idx, resid, &w);
// //                         left_idxidx.push(idx)
// //                     } else {
// //                         new_suff[1].update(idx, resid, &w);
// //                         right_idxidx.push(idx)
// //                     }
// //                 }
// //             },
// //             None => todo!(),
// //         }

// //             // (None, true) => {
// //             //     let n: usize = x[0].len();
// //             //     (0..n).for_each(|idx| {
// //             //         let node_idx = tree.predict_leaf_idx(x, idx) as u8;
// //             //         Self::update_suff(node_idx, idx, resid, w, &mut ret)
// //             //     });
// //             // }
// //             // (None, false) => {
// //             //     let n: usize = x[0].len();
// //             //     (0..n).for_each(|idx| {
// //             //         let node_idx = tree.predict_leaf_idx(x, idx) as u8;
// //             //         Self::update_suff_new(
// //             //             node_idx,
// //             //             idx,
// //             //             new,
// //             //             x,
// //             //             resid,
// //             //             w,
// //             //             &mut ret,
// //             //             new_suff,
// //             //             &mut left_idxidx,
// //             //             &mut right_idxidx
// //             //         )
// //             //     });
// //             // }
// //             // (Some(leaf_idx), true) => {
// //             //     leaf_idx
// //             //         .iter()
// //             //         .enumerate()
// //             //         .for_each(|(idx, &node_idx)| {
// //             //             Self::update_suff(node_idx, idx, resid, w, &mut ret)
// //             //         });
// //             // }
// //             // (Some(leaf_idx), false) => {
// //             //     leaf_idx
// //             //         .iter()
// //             //         .enumerate()
// //             //         .for_each(|(idx, &node_idx)| {
// //             //             Self::update_suff_new(
// //             //                 node_idx,
// //             //                 idx,
// //             //                 new,
// //             //                 x,
// //             //                 resid,
// //             //                 w,
// //             //                 &mut ret,
// //             //                 new_suff,
// //             //                 &mut left_idxidx,
// //             //                 &mut right_idxidx
// //             //             )
// //             //         });
// //             // }
// //         }
// //     }

// //     fn pr(&self) -> f64 {
// //         // self.x[1][1] as f64
// //         1.0
// //     }

// //     fn llh(&self) -> f64 {
// //         if self.node.is_none() {
// //             panic!("current propoal is not value: {:?}", &self)
// //         }
// //         let left_idx = self.node.unwrap().left_idx() as u8;
// //         let right_idx = self.node.unwrap().right_idx() as u8;
// //         let left = self.suff[&left_idx];
// //         let right = self.suff[&right_idx];
// //         let parent = Suff::merge(&left, &right);
// //         left.llh(self.sigma, self.tau) + right.llh(self.sigma, self.tau) - parent.llh(self.sigma, self.tau)
// //     }

// //     fn sample_mu(&mut self) {
// //         todo!()
// //     }
// // impl<'a> Sampler for GrowProposer<'a> {
// //     type Data = ();

// //     type Output = ();

// //     fn update(&mut self, data: &Self::Data) {}

// //     fn sample<R: Rng + ?Sized>(&mut self, rng: &mut R) -> Self::Output {
// //         self.proposal(rng);
// //         if self.node.is_none() {
// //             return ;
// //         }

// //         self.suff = Suff::get_all_suff(
// //             self.tree,
// //             self.node.unwrap(),
// //             self.x, self.resid, self.w
// //         );

// //         let accept_prob = f64::ln(rng.gen_range(0.0..1.0));

// //         if self.pr() + self.llh() < accept_prob {
// //             return ;
// //         }
// //         let node = self.node.unwrap();
// //         self.tree.grow(node, None)
// //         // let node = proposal.node.unwrap();
// //         // self.tree.grow(
// //         //     node.idx(), node.split(),
// //         //      0.0, 0.0,
// //         //      None, None
// //         // )
// //     }
// // }

// // // // trait TreeMutationProposer extends Serializable {

// // // // val topNode: LearningNode
// // // // val para: ForestPara
// // // // val proposer: Proposer
// // // // val treeId: Int
// // // // val sigmaSquareSampler: SigmaSquareSampler
// // // // val tauSampler: HalfNormSampler
// // // // val chainId: Int = para.chainId
// // // // val kind: String
// // // //
// // // // def logTransitionRatio: Double
// // // // def logLikelihoodRatio: Double
// // // // def logTreeRatio: Double

// // // // def logProbRatio: Double = logTransitionRatio + logLikelihoodRatio + logTreeRatio
// // // // def isAccept: Boolean = proposer.isQualified && (logProbRatio > proposer.logProbAccept)
// // // // def acceptedNode: LearningNode = if (isAccept) proposer.proposerNode else proposer.originNode

// // // // def update(): Unit = {
// // // //   updateSuffStat()
// // // //   refreshLeafRespAndUpdateResidual(acceptedNode)
// // // //   updateTree(acceptedNode)
// // // //   thin(proposer.proposerNode)
// // // //   thin(proposer.originNode)
// // // // }

// // // // def updateSuffStat(): Unit

// // // // def refreshLeafRespAndUpdateResidual(acceptedNode: LearningNode): Unit

// // // // def updateTree(acceptedNode: LearningNode): LearningNode

// // // // def logTransitionRatioForGrowOrPrune(): Double
// // // // def logTreeRatioForGrowOrPrune(): Double =

// // // // def thin(node: LearningNode): Unit =
// // // // def logProbNodeSplit(depth: Int): Double
// // // // def logProbNodeNotSplit(depth: Int): Double
// // // // def refreshRSum(node: LearningNode): Unit

// // // //   }



    // fn update_suff(
    //     node_idx: u8,
    //     idx: usize,
    //     resid: &Vec<f64>,
    //     w: &Weight,
    //     ret: &mut HashMap<u8, Suff>
    // ) {
    //     ret.entry(node_idx)
    //         .and_modify(|s| {
    //             s.update(idx as usize, resid, w);
    //         })
    //         .or_insert(Suff::default());
    // }


// // fn sample_mu(&mut self) ;
// // fn sample_mu(tree: &mut Tree, suff: &HashMap<u8, Suff>, resid: &mut Vec<f64>) {
// //     // debug_assert!(
// //     //     tree.get_all_leaf_idx().len() == suff.len(),
// //     //     "tree.get_all_leaf_idx().len():{}, suff.len():{}",
// //     //     tree.get_all_leaf_idx().len(), suff.len()
// //     // )
// //     let all_leaf_idx = tree.get_all_leaf_idx();
// //     all_leaf_idx.iter().for_each(|&idx| {
// //         let leaf = tree.get_node(idx);

// //     })

// // };

// pub mod sample_zs {
//     // Coefficients in rational approximations.
//     const ICDF_A: [f64; 6] = [
//         -3.969683028665376e1, 2.209460984245205e2, -2.759285104469687e2, 1.38357751867269e2,
//         -3.066479806614716e1, 2.506628277459239,
//     ];

//     const ICDF_B: [f64; 5] = [
//         -5.447609879822406e1, 1.615858368580409e2, -1.556989798598866e2, 6.680131188771972e1,
//         -1.328068155288572e1,
//     ];

//     const ICDF_C: [f64; 6] = [
//         -7.784894002430293e-3, -3.223964580411365e-1, -2.400758277161838, -2.549732539343734,
//         4.374664141464968, 2.938163982698783,
//     ];

//     const ICDF_D: [f64; 4] = [
//         7.784695709041462e-3, 3.224671290700398e-1, 2.445134137142996, 3.754408661907416,
//     ];

//     pub fn get_inv_cdf(d: f64) -> f64 {
//         const P_LOW: f64 = 0.02425;
//         const P_HIGH: f64 = 1.0 - P_LOW;
//         const EPSILON: f64 = 1e-14;

//         //kludge!!!
//         let d = f64::min(f64::max(EPSILON, d), 1.0 - EPSILON);

//         // Define break-points variable for result
//         let z = if d == 0.0 {
//             f64::NEG_INFINITY
//         } else if d == 1.0 {
//             f64::INFINITY
//         } else if f64::is_nan(d) || d < 0.0 || d > 1.0 {
//             f64::NAN
//         } else if d < P_LOW {
//             // Rational approximation for lower region:
//             let q: f64 = f64::sqrt(-2.0 * f64::ln(d));
//             let z =
//                 (((((ICDF_C[0] * q + ICDF_C[1]) * q + ICDF_C[2]) * q + ICDF_C[3]) * q + ICDF_C[4]) *
//                     q +
//                     ICDF_C[5]) /
//                 ((((ICDF_D[0] * q + ICDF_D[1]) * q + ICDF_D[2]) * q + ICDF_D[3]) * q + 1.0);
//             z
//         } else if P_HIGH < d {
//             // Rational approximation for upper region:
//             let q: f64 = f64::sqrt(-2.0 * f64::ln(1.0 - d));
//             let z =
//                 -(
//                     ((((ICDF_C[0] * q + ICDF_C[1]) * q + ICDF_C[2]) * q + ICDF_C[3]) * q +
//                         ICDF_C[4]) *
//                         q +
//                     ICDF_C[5]
//                 ) /
//                 ((((ICDF_D[0] * q + ICDF_D[1]) * q + ICDF_D[2]) * q + ICDF_D[3]) * q + 1.0);
//             z
//         } else {
//             // Rational approximation for central region:
//             let q: f64 = d - 0.5;
//             let r: f64 = q * q;
//             let z =
//                 ((((((ICDF_A[0] * r + ICDF_A[1]) * r + ICDF_A[2]) * r + ICDF_A[3]) * r +
//                     ICDF_A[4]) *
//                     r +
//                     ICDF_A[5]) *
//                     q) /
//                 (((((ICDF_B[0] * r + ICDF_B[1]) * r + ICDF_B[2]) * r + ICDF_B[3]) * r + ICDF_B[4]) *
//                     r +
//                     1.0);
//             z
//         };
//         return z;
//     }

//     pub fn normal_cdf(x: f64) -> f64 {
//         let sign = if x < 0.0 { -1.0 } else { 1.0 };
//         let x = f64::abs(x) / f64::sqrt(2.0);
//         // A&S formula 7.1.26
//         const NORM_CDF_P: f64 = 0.3275911;
//         let t: f64 = 1.0 / (1.0 + NORM_CDF_P * x);

//         // constants for the {@link normal_cdf} function
//         const NORM_CDF_A1: f64 = 0.254829592;
//         const NORM_CDF_A2: f64 = -0.284496736;
//         const NORM_CDF_A3: f64 = 1.421413741;
//         const NORM_CDF_A4: f64 = -1.453152027;
//         const NORM_CDF_A5: f64 = 1.061405429;
//         let y: f64 =
//             1.0 -
//             ((((NORM_CDF_A5 * t + NORM_CDF_A4) * t + NORM_CDF_A3) * t + NORM_CDF_A2) * t +
//                 NORM_CDF_A1) *
//                 t *
//                 f64::exp(-x * x);
//         let p: f64 = 0.5 * (1.0 + sign * y);

//         //kludge!!!
//         if p == 0.0 {
//             p + 1e-14
//         } else if p == 1.0 {
//             p - 1e-14
//         } else {
//             p
//         }
//     }

//     pub fn sample_zi(u: f64, g_x_i: f64, y_i: f64) -> f64 {
//         assert!(y_i == 0.0 || y_i == 1.0, "y_i in 0.0/1.0");
//         if y_i == 1.0 {
//             let p_i: f64 = normal_cdf(-g_x_i);
//             g_x_i + get_inv_cdf((1.0 - u) * p_i + u)
//         } else {
//             let p_i: f64 = normal_cdf(g_x_i);
//             g_x_i - get_inv_cdf((1.0 - u) * p_i + u)
//         }
//     }
// }


// #[cfg(test)]
// mod tests {
//     use super::*;

//     // fn mock_data() {

//     // }
//     #[test]
//     fn test_leaf_idx() {

//         #[derive(Debug)]
//         struct SRef<'a> {
//             v: &'a  mut Vec<i32>
//         }
//         let mut v = vec![1, 3];
//         let sref = SRef{v: &mut v};
//         // println!("sref: {:?}", &sref);
//         // v.push(10);
//         println!("sref: {:?}", &sref);
//         // assert_eq!(tree.get_all, vec![4, 10, 11, 3])
//     }
// }

// // impl<'a> GrowProposer<'a> {
// //     pub fn default(
// //         tree: &'a mut Tree,
// //         resid: &'a mut Vec<f64>,
// //         data_para: &'a DataAndPara,
// //         sigma: f64,
// //         tau: f64
// //     ) -> Self {
// //         Self {
// //             tree,
// //             resid,
// //             data_para,
// //             node: None,
// //             suff: HashMap::new(),
// //             sigma,
// //             tau,
// //         }
// //     }
// // }

// // impl<'a> Sampler for GrowProposer<'a> {
// //     type Data = ();

// //     type Output = ();

// //     fn update(&mut self, _data: &Self::Data) {}

// //     fn sample<R: Rng + ?Sized>(&mut self, rng: &mut R) {
// //         let is_feasible = self.proposal(rng);
// //         self.gather_suff();
// //         let accept_prob = f64::ln(rng.gen_range(0.0..1.0));
// //         let (pr, llh) = (self.pr(), self.llh());
// //         let is_accept = if is_feasible && pr + llh >= accept_prob {
// //             self.tree.grow(self.node.unwrap());
// //             true
// //         } else {
// //             false
// //         };
// //         self.tree
// //             .get_all_leaf_idx()
// //             .iter()
// //             .for_each(|&leaf_idx| {
// //                 let mut suff = self.suff.get_mut(&leaf_idx).expect("leaf_idx suff not exists");
// //                 let new_mu = suff.sample_mu(self.sigma, self.tau, rng);
// //                 self.tree.get_mut_node(leaf_idx).set_mu(new_mu);
// //                 suff.new_mu = new_mu;
// //             });
// //         self.update_resid(is_accept);
// //     }
// // }

// // impl<'a> TreeMutationProposer for GrowProposer<'a> {
// //     fn sample<R: Rng + ?Sized>(&mut self, rng: &mut R) {
// //         let is_feasible = self.proposal(rng);
// //         self.gather_suff();
// //         let accept_prob = f64::ln(rng.gen_range(0.0..1.0));
// //         let (pr, llh) = (self.pr(), self.llh());
// //         let is_accept = if is_feasible && pr + llh >= accept_prob {
// //             self.tree.grow(self.node.unwrap());
// //             true
// //         } else {
// //             false
// //         };
// //         self.tree
// //             .get_all_leaf_idx()
// //             .iter()
// //             .for_each(|&leaf_idx| {
// //                 let mut suff = self.suff.get_mut(&leaf_idx).expect("leaf_idx suff not exists");
// //                 let new_mu = suff.sample_mu(self.sigma, self.tau, rng);
// //                 self.tree.get_mut_node(leaf_idx).set_mu(new_mu);
// //                 suff.new_mu = new_mu;
// //             });
// //         self.update_resid(is_accept);
// //     }

// //     fn proposal<R: Rng + ?Sized>(&mut self, rng: &mut R) -> bool {
// //         let all_leaf_idx = self.tree.get_all_leaf_idx();
// //         if all_leaf_idx.len() == 0 {
// //             return false;
// //         }
// //         let node_idx = all_leaf_idx[rng.gen_range(0..all_leaf_idx.len())];
// //         let p = self.data_para.init_splits.len();
// //         let feat_id = rng.gen_range(0..p);
// //         let picked_split = self.data_para.init_splits[feat_id];
// //         let split = picked_split.rand_pick_sub_split(self.tree, node_idx, rng);
// //         if split.is_none() {
// //             false
// //         } else {
// //             self.node = Some(Node::internal(node_idx, split.unwrap()));
// //             true
// //         }
// //     }

// //     fn gather_suff(&mut self) {
// //         let mut ret: HashMap<u8, Suff> = HashMap::new();
// //         let (is_feasible, parent_idx, left_idx, right_idx, new) = match self.node {
// //             Some(new) => {
// //                 let parent_idx = new.idx();
// //                 let (left_idx, right_idx) = (new.left_idx(), new.right_idx());
// //                 (true, parent_idx, left_idx, right_idx, new)
// //             }
// //             None => { (false, 0, 0, 0, Node::leaf(0, 0.0)) }
// //         };
// //         self.tree.leaf_idx
// //             .iter_mut()
// //             .enumerate()
// //             .for_each(|(idx, node_idx_ptr)| {
// //                 let node_idx = *node_idx_ptr;
// //                 if is_feasible && node_idx == parent_idx {
// //                     if new.go_left(&self.data_para.x, idx) {
// //                         Self::update_suff(left_idx, idx, self.resid, &self.data_para.w, &mut self.suff);
// //                         *node_idx_ptr = left_idx;
// //                     } else {
// //                         Self::update_suff(right_idx, idx, self.resid, &self.data_para.w, &mut ret);
// //                         *node_idx_ptr = right_idx;
// //                     }
// //                 } else {
// //                     Self::update_suff(node_idx, idx, self.resid, &self.data_para.w, &mut ret);
// //                 }
// //             });
// //         self.suff.iter_mut().for_each(|(&leaf_idx, suff)| {
// //             if leaf_idx == left_idx || leaf_idx == right_idx {
// //                 suff.old_mu = self.tree.get_node(parent_idx).mu();
// //             } else {
// //                 suff.old_mu = self.tree.get_node(leaf_idx).mu();
// //             }
// //         });
// //     }

// //     fn update_resid(&mut self, is_accept: bool) {
// //         let (is_reject, parent_idx, left_idx, right_idx) = match self.node {
// //             Some(new) => {
// //                 let parent_idx = new.idx();
// //                 let (left_idx, right_idx) = (new.left_idx(), new.right_idx());
// //                 (!is_accept, parent_idx, left_idx, right_idx)
// //             }
// //             None => (false, 0, 0, 0),
// //         };
// //         self.tree.leaf_idx
// //             .iter_mut()
// //             .zip(self.resid.iter_mut())
// //             .for_each(|(node_idx_ptr, r)| {
// //                 let node_idx = *node_idx_ptr;
// //                 *r += self.suff[&node_idx].old_mu;
// //                 if is_reject && node_idx == left_idx && node_idx == right_idx {
// //                     *node_idx_ptr = parent_idx;
// //                 }
// //             });
// //     }

// //     fn pr(&self) -> f64 {
// //         match self.node {
// //             Some(_) => {
// //                 let d = self.node.unwrap().depth();
// //                 let num_leaf = self.tree.get_all_leaf_idx().len() as f64;
// //                 let num_nog = self.tree.get_all_nog_idx().len() as f64;
// //                 f64::ln(self.data_para.prob_prune / self.data_para.prob_grow) +
// //                     f64::ln(self.data_para.prob_depth_not(d + 1)) * 2.0 -
// //                     f64::ln(self.data_para.prob_depth_not(d)) +
// //                     f64::ln(num_leaf / num_nog)
// //             }
// //             None => 0.0,
// //         }
// //     }

// //     fn llh(&self) -> f64 {
// //         match self.node {
// //             Some(new) => {
// //                 let left_idx = new.left_idx() as u8;
// //                 let right_idx = new.right_idx() as u8;
// //                 let left = self.suff[&left_idx];
// //                 let right = self.suff[&right_idx];
// //                 let parent = Suff::merge(&left, &right);
// //                 left.llh(self.sigma, self.tau) +
// //                     right.llh(self.sigma, self.tau) -
// //                     parent.llh(self.sigma, self.tau)
// //             }
// //             None => 0.0,
// //         }
// //     }
// // }


// // // #[cfg(test)]
// // // mod tests {
// // //     use super::*;

// // //     #[test]
// // //     fn test_leaf_idx() {

// // //         #[derive(Debug)]
// // //         struct SRef<'a> {
// // //             v: &'a  mut Vec<i32>
// // //         }
// // //         let mut v = vec![1, 3];
// // //         let sref = SRef{v: &mut v};
// // //         // println!("sref: {:?}", &sref);
// // //         // v.push(10);
// // //         println!("sref: {:?}", &sref);
// // //         // assert_eq!(tree.get_all, vec![4, 10, 11, 3])
// // //     }
// // // }


// pub mod sampler {
//   // use statrs::distribution::{ Gamma, Continuous };
//   use rand_distr::{Distribution, Gamma};

//     pub trait Sampler {
//         type Output;
//         type Input;
//         fn chain_id(&self) -> u32;
//         fn value(&self) -> Self::Output;
//         fn update(&mut self, data: Self::Input);
//     }

//     #[derive(Debug)]
//     pub struct SigmaSquareSampler {
//         chain_id: u32,
//         n: u32,
//         lambda: f64,
//         nu: f64,
//         value: f64,
//     }

//     impl SigmaSquareSampler {
//         pub fn new(chain_id: u32, n: u32, lambda: f64, nu: f64, value: f64) -> Self { 
//           if n <= 0 {
//             panic!("n: {} should large 0", n)
//           };

//           if nu <= 0.0 {
//             panic!("nu: {} should large 0", nu)
//           };

//           if lambda <= 0.0 {
//             panic!("lambda: {} should large 0", lambda)
//           };

//           Self { chain_id, n, lambda, nu, value } 
//         }

//         pub fn default(n: u32) -> Self {
//           Self::new(0, n, 1.0, 3.0,  1.0)
//         }
//     }

//     impl Sampler for SigmaSquareSampler {
      
//         type Output = f64;
//         type Input = f64;
      
//         fn chain_id(&self) -> u32 {
//             self.chain_id
//         }

//         fn value(&self) -> Self::Output {
//             self.value
//         }

//         fn update(&mut self, resid_square_sum: Self::Input) {
//             let nu = (self.nu + self.n as f64) / 2.0;
//             let lambda = (self.nu * self.lambda + resid_square_sum) / 2.0;
//             let gamma = Gamma::new(nu, 1.0 / lambda).unwrap();
//             self.value = 1.0 / gamma.sample(&mut rand::thread_rng());
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use crate::sampler::sampler::{SigmaSquareSampler, Sampler};


//     #[test]
//     fn this_test_will_pass() {
        
//         let mut sigma_sqr = SigmaSquareSampler::default(10);
//         println!("init: {:?}", sigma_sqr);

//         sigma_sqr.update(100.0);
//         println!("update: {:?}", sigma_sqr);

//         sigma_sqr.update(10.0);
//         println!("update: {:?}", sigma_sqr);
        
//         assert!(true);
//     }
// }

// // import org.apache.commons.math3.distribution.GammaDistribution

// // class SigmaSquarePara(
// //   val numPoints: Long,
// //   val lambda: Double = 0.0,
// //   val q: Double = 0.9,
// //   val nu: Double = 3.0
// // ) extends Para

// // class SigmaSquareSampler(
// //   val para: SigmaSquarePara,
// //   val chainId: Int
// // ) extends Sampler {
// //   type T = Double
// //   type A = Double

// //   var value: T = 1.0

// //   def update(residualsSquareSum: A): Unit = {
// //     val nu = (para.nu + para.numPoints) / 2.0
// //     val lambda = (para.nu * para.lambda + residualsSquareSum) / 2.0
// //     value = 1.0 / new GammaDistribution(nu, 1.0 / lambda).sample()
// //   }

// //   def copy(newChainId: Int): SigmaSquareSampler = {

// //     val newSampler = new SigmaSquareSampler(para, newChainId)
// //     newSampler.value = value
// //     newSampler
// //   }

// //   override def toString: String = {
// //     s"chainId: $chainId, sigmaSquare: ${value.formatted("%.5f")}, "
// //   }
// // }


// use std::collections::HashMap;

// use rand::Rng;
// use crate::{ tree::{ tree::Tree, node::Node, suff::Suff }, DataAndPara::DataAndPara };

// use super::sampler::{ TreeMutationProposer, Sampler };

// #[derive(Debug)]
// pub struct ChangeProposer<'a> {
//     tree: &'a mut Tree,
//     resid: &'a mut Vec<f64>,
//     data_para: &'a DataAndPara,
//     node: Option<Node>,
//     suff: HashMap<u8, Suff>,
//     sigma: f64,
//     tau: f64,
// }

// impl<'a> ChangeProposer<'a> {
//     pub fn default(
//         tree: &'a mut Tree,
//         resid: &'a mut Vec<f64>,
//         data_para: &'a DataAndPara,
//         sigma: f64,
//         tau: f64
//     ) -> Self {
//         Self {
//             tree,
//             resid,
//             data_para,
//             node: None,
//             suff: HashMap::new(),
//             sigma,
//             tau,
//         }
//     }
// }

// impl<'a> Sampler for ChangeProposer<'a> {
//     type Data = ();

//     type Output = ();

//     fn update(&mut self, _data: &Self::Data) {}

//     fn sample<R: Rng + ?Sized>(&mut self, rng: &mut R) {
//         let is_feasible = self.proposal(rng);
//         self.gather_suff();
//         let accept_prob = f64::ln(rng.gen_range(0.0..1.0));
//         let (pr, llh) = (self.pr(), self.llh());
//         let is_accept = if is_feasible && pr + llh >= accept_prob {
//             self.tree.grow(self.node.unwrap());
//             true
//         } else {
//             false
//         };
//         self.tree
//             .get_all_leaf_idx()
//             .iter()
//             .for_each(|&leaf_idx| {
//                 let mut suff = self.suff.get_mut(&leaf_idx).expect("leaf_idx suff not exists");
//                 let new_mu = suff.sample_mu(self.sigma, self.tau, rng);
//                 self.tree.get_mut_node(leaf_idx).set_mu(new_mu);
//                 suff.new_mu = new_mu;
//             });
//         self.update_resid(is_accept);
//     }
// }

// impl<'a> TreeMutationProposer for PruneProposer<'a> {
//     fn sample<R: Rng + ?Sized>(&mut self, rng: &mut R) {
//         let is_feasible = self.proposal(rng);
//         self.gather_suff();
//         let accept_prob = f64::ln(rng.gen_range(0.0..1.0));
//         let (pr, llh) = (self.pr(), self.llh());
//         let is_accept = if is_feasible && pr + llh >= accept_prob {
//             self.tree.grow(self.node.unwrap());
//             true
//         } else {
//             false
//         };
//         self.tree
//             .get_all_leaf_idx()
//             .iter()
//             .for_each(|&leaf_idx| {
//                 Self::update_suff(*node_idx_ptr, idx, self.resid, &self.data_para.w, &mut ret);
//             });
//         self.update_resid(is_accept);
//     }

//     fn proposal<R: Rng + ?Sized>(&mut self, rng: &mut R) -> bool {
//         let all_leaf_idx = self.tree.get_all_leaf_idx();
//         if all_leaf_idx.len() == 0 {
//             return false;
//         }
//         let node_idx = all_leaf_idx[rng.gen_range(0..all_leaf_idx.len())];
//         let p = self.data_para.init_splits.len();
//         let feat_id = rng.gen_range(0..p);
//         let picked_split = self.data_para.init_splits[feat_id];
//         let split = picked_split.rand_pick_sub_split(self.tree, node_idx, rng);
//         if split.is_none() {
//             false
//         } else {
//             self.node = Some(Node::internal(node_idx, split.unwrap()));
//             true
//         }
//     }

//     fn gather_suff(&mut self) {
//         // let mut ret: HashMap<u8, Suff> = HashMap::new();
//         let (is_feasible, parent_idx, new) = match self.node {
//             Some(new) => {
//                 let parent_idx = new.idx();
//                 let (left_idx, right_idx) = (new.left_idx(), new.right_idx());
//                 (true, parent_idx, new)
//             }
//             None => { (false, 0, Node::leaf(0, 0.0)) }
//         };
//         self.tree.leaf_idx
//             .iter_mut()
//             .enumerate()
//             .for_each(|(idx, node_idx_ptr)| {
//                 let node_idx = *node_idx_ptr;
//                 Self::update_suff(node_idx, idx, self.resid, &self.data_para.w, &mut self.suff);
//             });
//         self.suff.iter_mut().for_each(|(&leaf_idx, suff)| {
//             if leaf_idx == left_idx || leaf_idx == right_idx {
//                 suff.old_mu = self.tree.get_node(parent_idx).mu();
//             } else {
//                 suff.old_mu = self.tree.get_node(leaf_idx).mu();
//             }
//         });
//     }

//     fn update_resid(&mut self, is_accept: bool) {
//         let (is_reject, parent_idx, left_idx, right_idx) = match self.node {
//             Some(new) => {
//                 let parent_idx = new.idx();
//                 let (left_idx, right_idx) = (new.left_idx(), new.right_idx());
//                 (!is_accept, parent_idx, left_idx, right_idx)
//             }
//             None => (false, 0, 0, 0),
//         };
//         self.tree.leaf_idx
//             .iter_mut()
//             .zip(self.resid.iter_mut())
//             .for_each(|(node_idx_ptr, r)| {
//                 let node_idx = *node_idx_ptr;
//                 *r += self.suff[&node_idx].old_mu;
//                 if is_reject && node_idx == left_idx && node_idx == right_idx {
//                     *node_idx_ptr = parent_idx;
//                 }
//             });
//     }

//     fn pr(&self) -> f64 {
//         0.0
//     }

//     fn llh(&self) -> f64 {
//         match self.node {
//             Some(new) => {
//                 let left_idx = new.left_idx() as u8;
//                 let right_idx = new.right_idx() as u8;
//                 let left = self.suff[&left_idx];
//                 let right = self.suff[&right_idx];
//                 let parent = Suff::merge(&left, &right);
//                 left.llh(self.sigma, self.tau) +
//                     right.llh(self.sigma, self.tau) -
//                     parent.llh(self.sigma, self.tau)
//             }
//             None => 0.0,
//         }
//     }
// }

// // #[cfg(test)]
// // mod tests {
// //     use super::*;

// //     #[test]
// //     fn test_leaf_idx() {

// //         #[derive(Debug)]
// //         struct SRef<'a> {
// //             v: &'a  mut Vec<i32>
// //         }
// //         let mut v = vec![1, 3];
// //         let sref = SRef{v: &mut v};
// //         // println!("sref: {:?}", &sref);
// //         // v.push(10);
// //         println!("sref: {:?}", &sref);
// //         // assert_eq!(tree.get_all, vec![4, 10, 11, 3])
// //     }
// // }

// if is_accept {
        // self.tree.grow_or_change(new);
        // let new_left = Suff::merge(&all_suff[&left_idx], &all_suff[&right_left_idx]);
        // let new_right = Suff::merge(&all_suff[&right_idx], &all_suff[&left_right_idx]);
        // let new_left = Suff::merge(&all_suff[&left_idx], &all_suff[&l_idx]);
        // let new_right = Suff::merge(&all_suff[&right_idx], &all_suff[&r_idx]);
        // all_suff.insert(left_idx, new_left);
        // all_suff.insert(right_idx, new_right);

        // self.sample_mu(all_suff, rng);

        // let left_new_mu = self.tree.mu(left_idx);
        // let right_new_mu = self.tree.mu(right_idx);

        // all_suff.get_mut(&l_idx).unwrap().set_new_mu(left_new_mu);
        // all_suff.get_mut(&r_idx).unwrap().set_new_mu(right_new_mu);
        // } else {
        //     let old_left = Suff::merge(&all_suff[&left_idx], &all_suff[&left_right_idx]);
        //     let old_right = Suff::merge(&all_suff[&right_idx], &all_suff[&right_left_idx]);
        //     all_suff.insert(left_idx, old_left);
        //     all_suff.insert(right_idx, old_right);
        //     self.sample_mu(all_suff, rng);

        //     let left_new_mu = self.tree.mu(left_idx);
        //     let right_new_mu = self.tree.mu(right_idx);

        //     all_suff.get_mut(&right_left_idx).unwrap().set_new_mu(right_new_mu);
        //     all_suff.get_mut(&left_right_idx).unwrap().set_new_mu(left_new_mu);
        // }