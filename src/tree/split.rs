use std::fmt;

use rand::Rng;

use super::{node::Node, tree::Tree};

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct Split {
    pub feat_idx: u16,
    idx_or_bit: i16,
}

const MAX_BITS: i16 = 15;

impl Split {
    pub fn new_categorical(feat_idx: u16, cats: Vec<i16>) -> Self {
        debug_assert!(
            cats.iter().min().unwrap() >= &0 && cats.iter().max().unwrap() < &MAX_BITS,
            "cats: {:?} should between 0 and {}",
            cats,
            MAX_BITS - 1
        );

        let mut bit = 1_i16 << MAX_BITS; //highest bit set to 1, so all categorical split < 0
        cats.iter().for_each(|&i| {
            bit |= 1_i16 << i;
        });
        Split {
            feat_idx,
            idx_or_bit: bit,
        }
    }

    pub fn new_continuous(feat_idx: u16, split_idx: i16) -> Self {
        debug_assert!(
            split_idx > 0 && split_idx < i16::MAX,
            "split_idx: {} should > 0 and < {}",
            split_idx,
            i16::MAX
        );
        Split {
            feat_idx,
            idx_or_bit: split_idx,
        }
    }

    pub fn new_leaf() -> Self {
        Split {
            feat_idx: u16::MAX,
            idx_or_bit: 0,
        }
    }

    #[inline]
    pub fn is_continous(&self) -> bool {
        self.idx_or_bit > 0
    }

    #[inline]
    pub fn is_categorical(&self) -> bool {
        !self.is_continous()
    }

    #[inline]
    pub fn is_leaf(&self) -> bool {
        self.idx_or_bit == 0
    }

    #[inline]
    pub fn go_left(&self, x: &Vec<Vec<i16>>, idx: usize) -> bool {
        let feat_val = x[self.feat_idx as usize][idx];
        debug_assert!(
            (self.is_continous() && feat_val >= 0)
                || (self.is_categorical() && feat_val >= 0 && feat_val < MAX_BITS),
            "feat_val: {}, self.is_continous(): {}",
            feat_val,
            self.is_continous()
        );

        if self.is_continous() {
            feat_val < self.idx_or_bit
        } else {
            (self.idx_or_bit & (1_i16 << feat_val)) != 0
        }
    }

    #[inline]
    pub fn go_right(&self, x: &Vec<Vec<i16>>, idx: usize) -> bool {
        !self.go_left(x, idx)
    }

    #[inline]
    pub fn go_left_v(&self, feat_val: i16) -> bool {
        // let feat_val = x[self.feat_idx as usize][idx];
        debug_assert!(
            (self.is_continous() && feat_val >= 0)
                || (self.is_categorical() && feat_val >= 0 && feat_val < MAX_BITS),
            "feat_val: {}, self.is_continous(): {}",
            feat_val,
            self.is_continous()
        );

        if self.is_continous() {
            feat_val < self.idx_or_bit
        } else {
            (self.idx_or_bit & (1_i16 << feat_val)) != 0
        }
    }

    #[inline]
    pub fn go_right_v(&self, feat_val: i16) -> bool {
        !self.go_left_v(feat_val)
    }

    fn cats(bit: i16) -> Vec<i16> {
        let ret: Vec<i16> = (0..MAX_BITS)
            .filter(|&ind| ((1_i16 << ind) & bit) != 0)
            .collect();
        ret
    }

    pub fn rand_pick_sub_split<R: Rng + ?Sized>(
        &self,
        tree: &Tree,
        node_idx: u8,
        rng: &mut R,
    ) -> Option<Split> {
        let feat_idx = self.feat_idx;
        if self.is_continous() {
            let (low, high) = self.avail_con_range(tree, node_idx);
            if high - low <= 1 {
                None
            } else {
                let picked_id = rng.gen_range(low..high);
                Some(Split::new_continuous(feat_idx, picked_id))
            }
        } else {
            let avail_cats = self.avail_cats(tree, node_idx);
            if avail_cats.len() <= 1 {
                None
            } else {
                let picked_cats = Self::rand_pick_cats_split(&avail_cats, rng);
                Some(Split::new_categorical(feat_idx, picked_cats))
            }
        }
    }

    pub fn is_splittable(&self, tree: &Tree, node_idx: u8) -> bool {
        if self.is_continous() {
            let (low, high) = self.avail_con_range(tree, node_idx);
            high - low > 1
        } else {
            self.avail_cats(tree, node_idx).len() > 1
        }
    }

    fn avail_con_range(&self, tree: &Tree, node_idx: u8) -> (i16, i16) {
        let mut low: i16 = 1;
        let mut high: i16 = self.idx_or_bit + 1;
        let mut parent_idx = node_idx;
        while parent_idx > 1 {
            let cur_idx = parent_idx;
            parent_idx = Node::get_parent_idx(parent_idx);
            let split = tree.get_node(parent_idx).split();
            if split.feat_idx != self.feat_idx {
                continue;
            }

            if Node::idx_is_left(cur_idx) {
                high = i16::min(split.idx_or_bit, high);
            } else {
                low = i16::max(split.idx_or_bit, low);
            }
        }
        (low, high)
    }

    fn avail_cats(self, tree: &Tree, node_idx: u8) -> Vec<i16> {
        let mut bit: i16 = self.idx_or_bit;
        let mut parent_idx = node_idx;
        while parent_idx > 1 {
            let cur_idx = parent_idx;
            parent_idx = Node::get_parent_idx(parent_idx);
            let split = tree.get_node(parent_idx).split();
            if split.feat_idx != self.feat_idx {
                continue;
            }

            if Node::idx_is_left(cur_idx) {
                bit &= split.idx_or_bit;
            } else {
                bit &= !split.idx_or_bit | (1_i16 << MAX_BITS);
            }
        }
        Split::cats(bit)
    }

    fn rand_pick_cats_split<R: Rng + ?Sized>(cats: &Vec<i16>, rng: &mut R) -> Vec<i16> {
        let avail_arity = cats.len();
        if avail_arity <= 1 {
            panic!("cats:{:?} should > 1", cats);
        }
        let num_splits = 1 << (avail_arity - 1);
        let picked_bit = rng.gen_range(1..num_splits);
        Split::cats(picked_bit)
            .iter()
            .map(|&id| cats[id as usize])
            .collect()
    }
}

impl fmt::Display for Split {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_leaf() {
            write!(f, "")
        } else if self.is_categorical() {
            write!(
                f,
                "x_{} in {:?}",
                self.feat_idx,
                Self::cats(self.idx_or_bit)
            )
        } else {
            write!(f, "x_{} <  {}", self.feat_idx, self.idx_or_bit)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use rand::thread_rng;

    use super::*;

    #[test]
    fn test_split() {
        println!("split0: {}", Split::new_leaf());

        let x: Vec<Vec<i16>> = vec![vec![1, 13, 10], vec![2, 3, 5]];
        let split1 = Split::new_continuous(0, 4);
        println!("split1: {}", split1);
        assert_eq!(split1.go_left(&x, 0), true);
        assert_eq!(split1.go_right(&x, 1), true);
        assert_eq!(split1.go_left(&x, 2), false);

        let split2 = Split::new_continuous(1, 4);
        assert_eq!(split2.go_left(&x, 0), true);
        assert_eq!(split2.go_right(&x, 1), false);
        assert_eq!(split2.go_left(&x, 2), false);

        let split3 = Split::new_categorical(0, vec![1_i16, 8_i16]);
        println!("split3: {}", split3);
        assert_eq!(split3.go_left(&x, 0), true);
        assert_eq!(split3.go_right(&x, 1), true);
        assert_eq!(split3.go_left(&x, 2), false);

        let split4 = Split::new_categorical(1, vec![1_i16, 8_i16]);
        assert_eq!(split4.go_left(&x, 0), false);
        assert_eq!(split4.go_right(&x, 1), true);
        assert_eq!(split4.go_left(&x, 2), false)
    }

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    enum CatsOrInt {
        Cat(Vec<i16>),
        Con(i16),
    }

    fn fmt(freq: &HashMap<CatsOrInt, f64>, w: usize, p: usize) -> String {
        let mut s: Vec<String> = freq
            .iter()
            .map(|(k, &v)| match k {
                CatsOrInt::Cat(cats) => {
                    let cats: Vec<String> = cats.iter().map(|&i| i.to_string()).collect();
                    format!("{: >w$}:{:.p$}", cats.join(""), v, w = w, p = p)
                }
                CatsOrInt::Con(k) => {
                    format!("{:0w$}:{:.p$}", *k, v, w = w, p = p)
                }
            })
            .collect::<Vec<String>>();
        s.sort();
        s.join("|")
    }

    #[test]
    fn test_rand_pick_con_split() {
        let init = Split::new_continuous(1, 19);

        let mut tree = Tree::new(10);
        tree.grow_or_change(Node::internal(1, Split::new_continuous(1, 13)));
        tree.grow_or_change(Node::internal(2, Split::new_continuous(0, 4)));
        tree.grow_or_change(Node::internal(5, Split::new_continuous(1, 8)));
        println!("{}", tree);

        let mut rng = thread_rng();
        let n = 1000000;
        let mut freq: HashMap<CatsOrInt, f64> = HashMap::new();
        (0..n).for_each(|_i| {
            let split_id = init.rand_pick_sub_split(&tree, 10, &mut rng).unwrap();
            freq.entry(CatsOrInt::Con(split_id.idx_or_bit))
                .and_modify(|f| {
                    *f += (1.0 as f64) / (n as f64);
                })
                .or_insert((1.0 as f64) / (n as f64));
        });
        println!("freq:{}", fmt(&freq, 2, 2));
        assert_eq!(
            fmt(&freq, 2, 2),
            "01:0.14|02:0.14|03:0.14|04:0.14|05:0.14|06:0.14|07:0.14"
        );

        let mut freq: HashMap<CatsOrInt, f64> = HashMap::new();
        (0..n).for_each(|_i| {
            let split_id = init.rand_pick_sub_split(&tree, 11, &mut rng).unwrap();
            freq.entry(CatsOrInt::Con(split_id.idx_or_bit))
                .and_modify(|f| {
                    *f += (1.0 as f64) / (n as f64);
                })
                .or_insert((1.0 as f64) / (n as f64));
        });
        println!("freq:{}", fmt(&freq, 2, 2));
        assert_eq!(fmt(&freq, 2, 2), "08:0.20|09:0.20|10:0.20|11:0.20|12:0.20");
    }

    #[test]
    fn test_rand_pick_cat_split() {
        let init = Split::new_categorical(1, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);

        let mut tree = Tree::new(10);
        tree.grow_or_change(Node::internal(
            1,
            Split::new_categorical(1, vec![0, 1, 3, 6, 8, 9, 11]),
        ));
        tree.grow_or_change(Node::internal(2, Split::new_continuous(0, 4)));
        tree.grow_or_change(Node::internal(5, Split::new_categorical(1, vec![0, 3, 9])));
        println!("{}", tree);

        let mut rng = thread_rng();
        let n = 1000000;
        let mut freq: HashMap<CatsOrInt, f64> = HashMap::new();
        (0..n).for_each(|_i| {
            let split_id = init.rand_pick_sub_split(&tree, 10, &mut rng).unwrap();
            freq.entry(CatsOrInt::Cat(Split::cats(split_id.idx_or_bit)))
                .and_modify(|f| {
                    *f += (1.0 as f64) / (n as f64);
                })
                .or_insert((1.0 as f64) / (n as f64));
        });
        println!("freq:{}", fmt(&freq, 2, 2));
        assert_eq!(fmt(&freq, 2, 2), " 0:0.33| 3:0.33|03:0.33");

        let mut freq: HashMap<CatsOrInt, f64> = HashMap::new();
        (0..n).for_each(|_i| {
            let split_id = init.rand_pick_sub_split(&tree, 11, &mut rng).unwrap();
            freq.entry(CatsOrInt::Cat(Split::cats(split_id.idx_or_bit)))
                .and_modify(|f| {
                    *f += (1.0 as f64) / (n as f64);
                })
                .or_insert((1.0 as f64) / (n as f64));
        });
        println!("freq:{}", fmt(&freq, 3, 2));
        assert_eq!(
            fmt(&freq, 3, 2),
            "  1:0.14|  6:0.14|  8:0.14| 16:0.14| 18:0.14| 68:0.14|168:0.14"
        );
    }
}
