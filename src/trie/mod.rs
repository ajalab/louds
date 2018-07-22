pub trait Trie<T> {
    fn has<K: AsRef<[T]>>(&self, key: K) -> bool;
}

mod louds;
mod vec;

pub use self::louds::TrieLouds;
pub use self::vec::TrieVec;
