// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//! # AMT
//!
//! See the [specs](https://github.com/ipld/specs/blob/master/data-structures/vector.md) for details.
//!
//! ## Introduction
//!
//! The IPLD Vector distributes ordered-by-construction values across a tree structure with
//! a pre-definable branching factor (`width`) that is consistent across all nodes in the tree.
//! The tree defines node "height" rather than "depth", with the values stored in leaf nodes
//! with a `height` of `0`.
//!
//! All these leaf nodes are organized into a collection of `height` `1` nodes
//! which contain links to the leaf nodes rather than values.
//! We continue to increment `height` until we have a single node as the root of the tree.
//!
//! An IPLD Vector containing fewer values than the `width` of the Vector can be represented
//! by a single root node, with a `height` of `0` and a `data` array containing only the values.
//! Once an IPLD Vector expands beyond `width`, we add additional sibling `height` `0` nodes
//! and reference them in a parent `height` `1` node.
//! Once the `height` `1` node expands beyond `width` child nodes, we perform the same operation
//! by adding a new `height` `2` node to organize `height` `1` nodes.
//!
//! ## Summary
//!
//! IPLD Vector's algorithm is roughly based on the data structure used by [PersistentVector in Clojure]
//! and referred to as a Bitmapped Vector Trie in [Scala's Vector].
//! It has roots in the concept of Array Mapped Tries (AMT), as outlined in [Phil Bagwell's papers] on the [subject].
//!
//! In these data structures, the indexing at each level of the trie comprises portions of the requested index.
//! By taking advantage of efficient bitwise operations, we can slice an index into segments which point us
//! through each level as we descend to the final value.
//! The concept is roughly similar to slicing a hash as outlined in the [IPLD HashMap](hashmap.md) specification,
//! except that we are slicing an index.
//!
//! One major difference with these data structures come from IPLD's minimal capacity to make use of
//! the efficiencies afforded by bitwise operations.
//! Without requiring bitwise operations, we don't have a strong need to align to byte or word boundaries
//! and can use non-bitwise operations to perform our indexing function.
//! Hence, the `width` of the IPLD Vector is variable (not a power of `2` as for the width of nodes
//! in the [HashMap](hashmap.md)), from a lower bound of `2`, for very tall trees that yield very
//! small blocks, up to very large values that yield shallow trees but very large blocks.
//! We leave the option of storing leaf values as CIDs or inline data up to the user, thereby
//! affording the possibility of tuning `width` to the desired block size with a traversal cost trade-off.
//!
//! IPLD Vectors don't implement a `map` as in [HashMap](hashmap.md) or as may be used in an AMT to
//! support compression for sparse arrays.
//! It is assumed that most IPLD Vector usage will not be for sparse data and if sparse storage is
//! needed that nodes containing empty `data` array slots are acceptable.
//! Note, however, that `Size` does not account for empty elements in this data structure.
//!
//! A fully inlined option for this data structure is not presented here as this can be achieved by
//! copying the data from an IPLD Vector into a new Vector whose `width` is at least the `Size` of
//! the original and ensuring that values are inlined when stored.
//! Therefore it is assumed that any nodes with `height` greater than `0` will have `data` arrays
//! containing only `CID`s which are links to child nodes.
//!
//! [PersistentVector in Clojure]: https://github.com/clojure/clojure/blob/master/src/jvm/clojure/lang/PersistentVector.java
//! [Scala's Vector]: https://github.com/scala/scala/blob/v2.13.0/src/library/scala/collection/immutable/Vector.scala
//! [Phil Bagwell's papers]: https://infoscience.epfl.ch/record/64394/files/triesearches.pdf
//! [subject]: http://lampwww.epfl.ch/papers/idealhashtrees.pdf
//!
//! ## Structure
//!
//! ### Parameters
//!
//! The only configurable parameter of an IPLD Vector is the `width`.
//! This parameter must be consistent across all nodes in a Vector.
//! Mutations cannot involve changes in `width` or joining multiple parts of a Vector with
//! differing `width` values.
//!
//! `width` must be an integer, of at least `2`.
//!
//! ### Node properties
//!
//! Each node in an IPLD vector stores the `width`, the `height` of the node,
//! starting from `0` where values are stored, and a `data` array to contain values (for `height` `0`),
//! or child node CIDs (for `height`s above `1`).
//!
//! ### Schema
//!
//! ```ipldsch
//! type Vector struct {
//!   width Int
//!   height Int
//!   data [ Value ]
//! }
//!
//! type Value union {
//!   | Link link
//!   | Bool bool
//!   | String string
//!   | Bytes bytes
//!   | Int int
//!   | Float float
//!   | Map map
//!   | List list
//! } representation kinded
//! ```
//!
//! ### Constraints
//!
//! * Non-leaf (`height` greater than `0`) nodes only contain `Link`s to other `Vector` nodes in their `data` array.
//! * `width` must be consistent across all nodes in a Vector.
//! * `height` must be at least `0`.
//! * `data` can contain between `1` and `width` elements.
//!   For the special case of the empty Vector, a single root node may have `0` elements in `data`.
//!

#![deny(missing_docs)]

mod amt;
mod bitmap;
mod error;
mod node;
mod root;

pub use self::amt::IpldAmt;
pub use self::error::IpldAmtError;
pub use self::root::Root;

/// The only configurable parameter of an IPLD Vector.
/// This parameter must be consistent across all nodes in a Vector.
///
/// Mutations cannot involve changes in width or
/// joining multiple parts of a Vector with differing width values.
///
/// `WIDTH` must be an integer, of at least 2.
pub const WIDTH: usize = 8;

// Max size of leaf values before root overflow.
// ============================================================================
// Height
// ↓
// 0:    [1 2 3]
//
// Max size of leaf nodes before root overflow: 1
// Max size of leaf values before root overflow: 3
//
// ============================================================================
// Height
// ↓
// 1:            [a b c]
//          ┌─────┘ │ └─────┐
// 0:    [1 2 3] [4 5 6] [7 8 9]
//
// Max size of leaf nodes before root overflow: 3
// Max size of leaf values before root overflow: 9
//
// ============================================================================
// Height
// ↓
// 2:                                       [A  B  C]
//                 ┌─────────────────────────┘  │  └─────────────────────────────┐
// 1:            [a b c]                    [d  e  f]                        [g  h  i]
//          ┌─────┘ │ └─────┐        ┌───────┘  │  └────────┐         ┌───────┘  │  └────────┐
// 0:    [1 2 3] [4 5 6] [7 8 9] [10 11 12] [13 14 15] [16 17 18] [19 20 21] [22 23 24] [25 26 27]
//
// Max size of leaf nodes before root overflow: 9
// Max size of leaf values before root overflow: 27
#[inline]
fn max_leaf_value_size_for(height: u64) -> usize {
    WIDTH * max_leaf_node_size_for(height)
}

// Max size of leaf nodes before root overflow.
#[inline]
fn max_leaf_node_size_for(height: u64) -> usize {
    WIDTH.pow(height as u32)
}
