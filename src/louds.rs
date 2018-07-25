use fid::{BitVector, FID};

pub struct Louds(BitVector);

impl Louds {
    /// Returns an empty LOUDS tree.
    pub fn new() -> Louds {
        let mut bv = BitVector::new();
        bv.push(true);
        bv.push(false);
        Louds(bv)
    }

    /// Push a node which has `d` children.
    pub fn push_node(&mut self, d: usize) {
        for _ in 0..d {
            self.0.push(true);
        }
        self.0.push(false);
    }

    /// Push a bit.
    pub fn push_bit(&mut self, b: bool) {
        self.0.push(b);
    }

    /// Returns the first child of the i-th node in breadth-first order.
    pub fn first_child(&self, i: usize) -> Option<usize> {
        let first_child_pos = self.0.select0(i as u64) + 1;

        if self.0.get(first_child_pos) {
            Some(self.0.rank1(first_child_pos) as usize)
        } else {
            None
        }
    }

    /// Returns the last child of the i-th node in breadth-first order.
    pub fn last_child(&self, i: usize) -> Option<usize> {
        let last_child_pos = self.0.select0(i as u64 + 1) - 1;

        if self.0.get(last_child_pos) {
            Some(self.0.rank1(last_child_pos) as usize)
        } else {
            None
        }
    }

    /// Returns `true` if the i-th node is a leaf node (has no children).
    pub fn is_leaf(&self, i: usize) -> bool {
        let first_child_pos = self.0.select0(i as u64) + 1;
        !self.0.get(first_child_pos)
    }

    /// Returns the degree (# of children) of the i-th node.
    pub fn degree(&self, i: usize) -> usize {
        let first_child_pos = self.0.select0(i as u64) + 1;

        if self.0.get(first_child_pos) {
            let last_child_pos = self.0.select0(i as u64 + 1) - 1;
            (last_child_pos - first_child_pos + 1) as usize
        } else {
            0
        }
    }

    /// Returns the range `(s, e)` of children of the i-th node.
    ///
    /// The first value `s` corresponds to the first child,
    /// and `e` corresponds to the last child.
    ///
    /// `l.range_children()` is equivalent to `(l.first_child(), l.last_child())` but
    /// `range_children` is faster.
    pub fn range_children(&self, i: usize) -> Option<(usize, usize)> {
        let first_child_pos = self.0.select0(i as u64) + 1;

        if self.0.get(first_child_pos) {
            let last_child_pos = self.0.select0(i as u64 + 1) - 1;
            let s = self.0.rank1(first_child_pos);
            let d = last_child_pos - first_child_pos;
            Some((s as usize, (s + d) as usize))
        } else {
            None
        }
    }

    /// Returns the next sibling of the i-th node.
    pub fn sibling(&self, i: usize) -> Option<usize> {
        let p = self.0.select1(i as u64);
        if self.0.get(p + 1) {
            Some(i + 1)
        } else {
            None
        }
    }

    /// Returns the k-th child of the i-th node.
    pub fn child(&self, i: usize, k: usize) -> Option<usize> {
        self.range_children(i)
            .and_then(|(s, e)| if s + k <= e { Some(s + k) } else { None })
    }

    /// Returns the parent of the i-th node.
    pub fn parent(&self, i: usize) -> Option<usize> {
        if i > 0 {
            let p = self.0.select1(i as u64);
            Some(self.0.rank0(p) as usize - 1)
        } else {
            None
        }
    }

    /// Returns the depth of the node `i`.
    pub fn depth(&self, i: usize) -> usize {
        let mut j = i as u64;
        let mut d = 0;
        while j > 0 {
            let p = self.0.select1(j);
            j = self.0.rank0(p) - 1;
            d = d + 1;
        }
        d
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NODES_LIGHT: &[usize] = &[2, 3, 2, 0, 2, 1, 1, 0, 0, 0, 0, 0];
    const FIRST_CHILDREN_LIGHT: &[Option<usize>] = &[
        Some(1),
        Some(3),
        Some(6),
        None,
        Some(8),
        Some(10),
        Some(11),
        None,
        None,
        None,
        None,
        None,
    ];

    const LAST_CHILDREN_LIGHT: &[Option<usize>] = &[
        Some(2),
        Some(5),
        Some(7),
        None,
        Some(9),
        Some(10),
        Some(11),
        None,
        None,
        None,
        None,
        None,
    ];

    const SIBLINGS_LIGHT: &[Option<usize>] = &[
        None,
        Some(2),
        None,
        Some(4),
        Some(5),
        None,
        Some(7),
        None,
        Some(9),
        None,
        None,
        None,
    ];

    const DEPTH_LIGHT: &[usize] = &[0, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3, 3];

    const PARENTS_LIGHT: &[Option<usize>] = &[
        None,
        Some(0),
        Some(0),
        Some(1),
        Some(1),
        Some(1),
        Some(2),
        Some(2),
        Some(4),
        Some(4),
        Some(5),
        Some(6),
    ];

    fn get_tree_light() -> Louds {
        let mut louds = Louds::new();
        for &d in NODES_LIGHT {
            louds.push_node(d);
        }
        louds
    }

    #[test]
    fn test_first_child() {
        let louds = get_tree_light();

        for i in 0..NODES_LIGHT.len() {
            assert_eq!(
                louds.first_child(i),
                FIRST_CHILDREN_LIGHT[i],
                "The first child of node {} is {:?}",
                i,
                FIRST_CHILDREN_LIGHT[i]
            );
        }
    }

    #[test]
    fn test_last_child() {
        let louds = get_tree_light();

        for i in 0..NODES_LIGHT.len() {
            assert_eq!(
                louds.last_child(i),
                LAST_CHILDREN_LIGHT[i],
                "The last child of node {} is {:?}",
                i,
                LAST_CHILDREN_LIGHT[i]
            );
        }
    }

    #[test]
    fn test_is_leaf() {
        let louds = get_tree_light();

        for i in 0..NODES_LIGHT.len() {
            let ans = NODES_LIGHT[i] == 0;
            assert_eq!(
                louds.is_leaf(i),
                ans,
                "The node {} is {}a leaf node.",
                i,
                if ans { "" } else { "not " }
            );
        }
    }

    #[test]
    fn test_degree() {
        let louds = get_tree_light();

        for (i, d) in NODES_LIGHT.iter().enumerate() {
            assert_eq!(louds.degree(i), *d, "The degree of node {} is {}", i, *d);
        }
    }

    #[test]
    fn test_range_children() {
        let louds = get_tree_light();

        for i in 0..NODES_LIGHT.len() {
            let f = FIRST_CHILDREN_LIGHT[i];
            let l = LAST_CHILDREN_LIGHT[i];
            let ans = f.and_then(|a| l.map(|b| (a, b)));
            assert_eq!(
                louds.range_children(i),
                ans,
                "The last child of node {} is {:?}",
                i,
                ans
            );
        }
    }

    #[test]
    fn test_sibling() {
        let louds = get_tree_light();

        for i in 0..NODES_LIGHT.len() {
            assert_eq!(
                louds.sibling(i),
                SIBLINGS_LIGHT[i],
                "The next sibling of node {} is {:?}",
                i,
                SIBLINGS_LIGHT[i]
            );
        }
    }

    #[test]
    fn test_child() {
        let louds = get_tree_light();
        for i in 0..NODES_LIGHT.len() {
            let d = louds.degree(i);
            let mut ans = louds.first_child(i);
            for k in 0..=d {
                let c = louds.child(i, k);
                assert_eq!(c, ans, "The {}-th child of node {} is {:?}", k, i, ans);
                ans = ans.and_then(|i| louds.sibling(i));
            }
        }
    }

    #[test]
    fn test_parent() {
        let louds = get_tree_light();

        for i in 0..NODES_LIGHT.len() {
            assert_eq!(
                louds.parent(i),
                PARENTS_LIGHT[i],
                "The parent of node {} is {:?}",
                i,
                PARENTS_LIGHT[i]
            );
        }
    }

    #[test]
    fn test_depth() {
        let louds = get_tree_light();

        for i in 0..NODES_LIGHT.len() {
            assert_eq!(
                louds.depth(i),
                DEPTH_LIGHT[i],
                "The depth of node {} is {:?}",
                i,
                DEPTH_LIGHT[i]
            );
        }
    }
}
