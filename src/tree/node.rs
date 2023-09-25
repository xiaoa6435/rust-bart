use std::fmt;

use super::split::Split;

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct Node {
    idx: u8,
    split: Split,
}

impl Node {
    #[inline]
    pub fn leaf(idx: u8) -> Self {
        debug_assert!(idx != 0, "idx:{} > 0", idx);
        Node {
            idx,
            split: Split::new_leaf(),
        }
    }

    #[inline]
    pub fn internal(idx: u8, split: Split) -> Self {
        debug_assert!(idx != 0, "idx:{} > 0", idx);
        Node { idx, split }
    }

    #[inline]
    pub fn as_leaf(&self) -> Self {
        Self::leaf(self.idx())
    }

    #[inline]
    pub fn as_internal(&self, split: Split) -> Self {
        Self::internal(self.idx(), split)
    }

    #[inline]
    pub fn is_leaf(&self) -> bool {
        self.split.is_leaf()
    }

    #[inline]
    pub fn is_internal(&self) -> bool {
        !self.is_leaf()
    }

    #[inline]
    pub fn idx(&self) -> u8 {
        self.idx
    }

    #[inline]
    pub fn is_left(&self) -> bool {
        Self::idx_is_left(self.idx())
    }

    #[inline]
    pub fn is_right(&self) -> bool {
        Self::idx_is_right(self.idx())
    }

    #[inline]
    pub fn left_idx(&self) -> u8 {
        Self::get_left_idx(self.idx())
    }

    #[inline]
    pub fn right_idx(&self) -> u8 {
        Self::get_right_idx(self.idx())
    }

    #[inline]
    pub fn child_idx(&self, x: &Vec<Vec<i16>>, idx: usize) -> u8 {
        if self.go_left(x, idx) {
            self.left_idx()
        } else {
            self.right_idx()
        }
    }

    #[inline]
    pub fn parent_idx(&self) -> u8 {
        Self::get_parent_idx(self.idx())
    }

    #[inline]
    pub fn depth(&self) -> u8 {
        Self::get_idx_depth(self.idx())
    }

    #[inline]
    pub fn split(&self) -> Split {
        debug_assert!(self.is_internal(), "only for internal node: {:?}", &self);
        self.split
    }

    #[inline]
    pub fn set_split(&mut self, split: Split) {
        debug_assert!(self.is_internal(), "only for internal node: {:?}", &self);
        self.split = split;
    }

    #[inline]
    pub fn go_left(&self, x: &Vec<Vec<i16>>, idx: usize) -> bool {
        debug_assert!(
            self.is_internal(),
            "go_left/right only for internal node: {:?}",
            &self
        );
        self.split.go_left(x, idx)
    }

    #[inline]
    pub fn go_right(&self, x: &Vec<Vec<i16>>, idx: usize) -> bool {
        !self.go_left(x, idx)
    }

    #[inline]
    pub fn go_left_v(&self, feat_val: i16) -> bool {
        debug_assert!(
            self.is_internal(),
            "go_left/right only for internal node: {:?}",
            &self
        );
        self.split.go_left_v(feat_val)
    }

    #[inline]
    pub fn go_right_v(&self, feat_val: i16) -> bool {
        !self.go_left_v(feat_val)
    }

    #[inline]
    pub fn get_idx_depth(idx: u8) -> u8 {
        (idx as f64).log2().floor() as u8
    }

    #[inline]
    pub fn idx_is_left(idx: u8) -> bool {
        idx % 2 == 0
    }

    #[inline]
    pub fn idx_is_right(idx: u8) -> bool {
        idx % 2 == 1
    }

    #[inline]
    pub fn get_left_idx(idx: u8) -> u8 {
        idx * 2
    }

    #[inline]
    pub fn get_right_idx(idx: u8) -> u8 {
        idx * 2 + 1
    }

    #[inline]
    pub fn get_parent_idx(idx: u8) -> u8 {
        idx / 2
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "id:{}:{}", self.idx, self.split)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_idx() {
        assert_eq!(Node::leaf(1).depth(), 0);

        let leaf = Node::leaf(4);
        assert_eq!(leaf.is_leaf(), true);
        assert_eq!(leaf.depth(), 2);

        assert_eq!(leaf.is_left(), true);
        assert_eq!(leaf.is_right(), false);

        assert_eq!(leaf.left_idx(), 8);
        assert_eq!(leaf.right_idx(), 9);
        assert_eq!(leaf.parent_idx(), 2);

        let mut internal = Node::internal(7, Split::new_continuous(1, 3));
        assert_eq!(internal.is_internal(), true);
        assert_eq!(internal.depth(), 2);

        assert_eq!(internal.split(), Split::new_continuous(1, 3));
        internal.set_split(Split::new_continuous(2, 5));
        assert_eq!(internal.split(), Split::new_continuous(2, 5));
    }

    #[test]
    #[should_panic]
    fn test_go_left() {
        let node = Node::leaf(1);
        let x: Vec<Vec<i16>> = vec![vec![1, 13, 10], vec![2, 3, 5]];
        node.go_left(&x, 1);
    }

    #[test]
    #[should_panic]
    fn test_split() {
        let node = Node::leaf(1);
        node.split();
    }
}
