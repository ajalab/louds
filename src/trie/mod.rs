//! Trie
//!
//! This sub-crate provides Trie implementations.

/// An ordered tree to manage a set of keys. It can answer whether it contains a given key or not.
///
/// Keys are any values that can be converted to a slice of alphabets (type `T`).
pub trait Trie<T> {
    /// Returns `true` if the trie contains `key`.
    fn has<K: AsRef<[T]>>(&self, key: K) -> bool;
}

mod louds;
mod vec;

pub use self::louds::TrieLouds;
pub use self::vec::TrieVec;
