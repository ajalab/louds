//! LOUDS (level order unary degree sequence) Tree implementation for Rust
//!
//! This crate provides a succinct data structure for ordered trees
//! that supports constant-time tree traversal operations.
//!
//! In LOUDS, a tree structure containing n nodes is repsresented as
//! a bit sequence of length 2n + 1.
//! We compress the sequence by using [fid](https://crates.io/crates/fid).
//!
//! It also includes [Trie](https://en.wikipedia.org/wiki/Trie) implementation with LOUDS.
//!
//! # Examples
//!
//! This example creates the following ordered tree.
//! Nodes are identified by breadth-first numbering.
//! ```text
//!        0
//!     /     \
//!    1       2
//!  / | \    / \
//! 3  4  5  6   7
//!   / \ |  |
//!   8 9 10 11
//! ```
//!
//! ```rust
//! extern crate louds;
//! use louds::Louds;
//!
//! // Degrees (# of children) of each node
//! let degrees = &[ 2, 3, 2, 0, 2, 1, 1, 0, 0, 0, 0, 0 ];
//! let mut louds = Louds::new();
//! for &d in degrees {
//!     louds.push_node(d);
//! }
//!
//! // Tree traversal operations (move to parent/children/sibling)
//! // are supported in constant-time.
//! assert_eq!(louds.first_child(1), Some(3));
//! assert_eq!(louds.first_child(3), None);
//! assert_eq!(louds.last_child(2), Some(7));
//! assert_eq!(louds.last_child(7), None);
//! assert_eq!(louds.child(1, 1), Some(4));
//! assert_eq!(louds.parent(4), Some(1));
//! assert_eq!(louds.sibling(4), Some(5));
//! assert_eq!(louds.degree(4), 2);
//!
//! // Computing depth of a node takes time proportional to
//! // the height of the tree.
//! assert_eq!(louds.depth(4), 2);
//! ```

extern crate fid;

mod louds;
pub mod trie;

pub use louds::Louds;
