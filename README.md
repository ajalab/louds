# LOUDS

[![Crates.io](https://img.shields.io/crates/v/louds.svg)](https://crates.io/crates/louds)
[![docs.rs](https://docs.rs/louds/badge.svg)](https://docs.rs/louds)

This crate provides a succinct data structure called LOUDS (level order unary degree sequence). LOUDS represents an ordered tree structure and supports almost constant-time tree traversal operations.

In LOUDS, a tree structure containing n nodes is repsresented as
a bit sequence of length 2n + 1.
We compress the sequence by using [fid](https://crates.io/crates/fid).

This crate also includes [Trie](https://en.wikipedia.org/wiki/Trie) implementation with LOUDS.

## Usage

Add this to your `Cargo.toml`.

```toml
[dependencies]
louds = "0.1.0"
```

## Examples

### Ordered Tree

```text
       0
    /     \
   1       2
 / | \    / \
3  4  5  6   7
  / \ |  |
  8 9 10 11
```
```rust
extern crate louds;
use louds::Louds;

// Create LOUDS tree by pushing degree (# of children) of
// each node in breadth-first order.
let degrees = &[ 2, 3, 2, 0, 2, 1, 1, 0, 0, 0, 0, 0 ];
let mut louds = Louds::new();
for &d in degrees {
    louds.push_node(d);
}

// Tree traversal operations (move to parent/children/sibling)
// are supported in constant-time.
assert_eq!(louds.first_child(1), Some(3));
assert_eq!(louds.first_child(3), None);
assert_eq!(louds.last_child(2), Some(7));
assert_eq!(louds.last_child(7), None);
assert_eq!(louds.child(1, 1), Some(4));
assert_eq!(louds.parent(4), Some(1));
assert_eq!(louds.sibling(4), Some(5));
assert_eq!(louds.degree(4), 2);

// Computing depth of a node takes time proportional to
// the height of the tree.
assert_eq!(louds.depth(4), 2);
```

## Credits

LOUDS representation was first proposed in [1].

[1] G. Jacobson(1989). Space-efficient Static Trees and Graphs. In Proc. IEEE FOCS, pages 549–554.

[2] 定兼 邦彦(2018). 簡潔データ構造. 共立出版.
