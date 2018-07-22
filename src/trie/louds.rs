use fid::BitArray;
use trie::{Trie, TrieVec};
use Louds;

pub struct TrieLouds<T> {
    louds: Louds,
    labels: Vec<T>,
    terminal: BitArray,
}

impl<T: Clone> From<TrieVec<T>> for TrieLouds<T> {
    fn from(t: TrieVec<T>) -> Self {
        let mut louds = Louds::new();
        let mut labels = Vec::new();
        let mut terminal = BitArray::new(0);

        for (i, subt) in t.bf_iter().enumerate() {
            for (c, _) in subt.children() {
                louds.push_bit(true);
                labels.push(c.clone());
            }
            terminal.set_bit(i, subt.terminal());
            louds.push_bit(false);
        }
        TrieLouds {
            louds: louds,
            labels: labels,
            terminal: terminal,
        }
    }
}

impl<T: Eq + PartialOrd + Ord> Trie<T> for TrieLouds<T> {
    fn has<K: AsRef<[T]>>(&self, key: K) -> bool {
        let mut i = 0;
        for c in key.as_ref() {
            let (s, e) = match self.louds.range_children(i) {
                Some(p) => p,
                None => return false,
            };

            let labels = &self.labels[s - 1..=e - 1];
            match labels.binary_search(c) {
                Ok(pos) => i = s + pos,
                Err(_) => return false,
            };
        }
        self.terminal.get_bit(i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_trie_louds() {
        let mut t = TrieVec::new();
        let keys = &["to", "tea", "ten", "i", "in", "inn", "we"];
        let keys_not = &["te", "inno", "web", "hoge", ""];
        for key in keys {
            t.insert(key.chars().collect::<Vec<_>>());
        }

        let t = TrieLouds::from(t);

        for key in keys {
            let key = key.chars().collect::<Vec<_>>();
            assert!(t.has(&key), "t should have key '{:?}'", key);
        }
        for key in keys_not {
            let key = key.chars().collect::<Vec<_>>();
            assert!(!t.has(&key), "t should not have key '{:?}'", key);
        }
    }
}
