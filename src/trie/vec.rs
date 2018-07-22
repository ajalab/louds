use std::collections::VecDeque;
use trie::Trie;

/// Naive trie implementation supporting dynamic insertion.
#[derive(Debug)]
pub struct TrieVec<T> {
    children: Vec<(T, Box<TrieVec<T>>)>,
    terminal: bool,
}

impl<'a, T> TrieVec<T> {
    pub fn new() -> TrieVec<T> {
        TrieVec {
            children: Vec::new(),
            terminal: false,
        }
    }
    pub fn bf_iter(&'a self) -> BFIter<'a, T> {
        BFIter::new(&self)
    }
    pub fn children(&self) -> &Vec<(T, Box<Self>)> {
        &self.children
    }
    pub fn terminal(&self) -> bool {
        self.terminal
    }
    pub fn children_len(&self) -> usize {
        self.children.len()
    }
}

impl<T: Eq + PartialOrd + Ord + Clone> TrieVec<T> {
    pub fn insert<K: AsRef<[T]>>(&mut self, key: K) {
        let mut t = self;
        for c in key.as_ref() {
            let mut next_pos = t.children.len();
            let mut found = false;
            for (i, (c2, _)) in t.children.iter().enumerate() {
                if c <= c2 {
                    next_pos = i;
                    found = c == c2;
                    break;
                }
            }
            if !found {
                t.children
                    .insert(next_pos, (c.clone(), Box::new(TrieVec::new())));
            }
            t = &mut { t }.children[next_pos].1;
        }
        t.terminal = true;
    }
}

impl<T: Eq + PartialOrd + Ord> Trie<T> for TrieVec<T> {
    fn has<K: AsRef<[T]>>(&self, key: K) -> bool {
        let mut t = self;
        for c in key.as_ref() {
            match t.children.binary_search_by(|(c2, _)| c2.cmp(c)) {
                Ok(pos) => {
                    t = &t.children[pos].1;
                }
                Err(_) => {
                    return false;
                }
            }
        }
        t.terminal
    }
}

pub struct BFIter<'a, T: 'a> {
    queue: VecDeque<&'a TrieVec<T>>,
}

impl<'a, T> BFIter<'a, T> {
    fn new(t: &'a TrieVec<T>) -> Self {
        let mut queue = VecDeque::new();
        queue.push_back(t);
        BFIter { queue: queue }
    }
}

impl<'a, T> Iterator for BFIter<'a, T> {
    type Item = &'a TrieVec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop_front().map(|node| {
            for (_, t) in &node.children {
                self.queue.push_back(t);
            }
            node
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_trie_vec() {
        let mut t = TrieVec::new();
        let keys = &["to", "tea", "ten", "i", "in", "inn", "we"];
        let keys_not = &["te", "inno", "web", "hoge", ""];
        for key in keys {
            t.insert(key);
        }

        for key in keys {
            assert!(t.has(key), "t should have key '{}'", key);
        }
        for key in keys_not {
            assert!(!t.has(key), "t should not have key '{}'", key);
        }
    }
}
