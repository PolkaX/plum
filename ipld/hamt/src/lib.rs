// Copyright 2019-2020 PolkaX Authors. Licensed under GPL-3.0.

//! # HAMT
//!
//! See [IPLD specs](https://github.com/ipld/specs/blob/master/data-structures/hashmap.md) for details.
//!
//! ## Introduction
//!
//! The IPLD HashMap provides multi-block key/value storage and implements
//! the Map [kind](/data-model-layer/data-model.md#kinds) as an advanced data layout
//! in the IPLD type system.
//!
//! The IPLD HashMap is constructed as a [hash array mapped trie (HAMT)] with buckets
//! for value storage and [CHAMP] mutation semantics.
//! The CHAMP invariant and mutation rules provide us with the ability to maintain
//! canonical forms given any set of keys and their values,
//! regardless of insertion order and intermediate data insertion and deletion.
//! Therefore, for any given set of keys and their values,
//! a consistent IPLD HashMap configuration and block encoding,
//! the root node should always produce the same content identifier (CID).
//!
//! [hash array mapped trie (HAMT)]: https://en.wikipedia.org/wiki/Hash_array_mapped_trie
//! [CHAMP]: https://michael.steindorfer.name/publications/oopsla15.pdf
//!
//! ## Useful references
//!
//! * [Fast And Space Efficient Trie Searches] by Phil Bagwell, 2000, and [Ideal Hash Trees] by Phil Bagwell, 2001, introduce the AMT and HAMT concepts.
//! * [CHAMP paper] presented at Oopsla 2015 by Steinforder & Vinju
//! * [Java implementation] accompanying the original CHAMP paper (see [TrieMap_5Bits.java] and other TrieMap files in the same directory).
//! * [Optimizing Hash-Array Mapped Tries for Fast and Lean Immutable JVM Collections] a high-level description of HAMT data structures in general and the specifics of CHAMP.
//! * Peergos [CHAMP Java] implementation
//! * [IAMap] JavaScript implementation of the algorithm
//! * [ipld-hashmap] JavaScript IPLD frontend to IAMap with a mutable API
//! * [go-hamt-ipld] Go implementation, not strictly aligned to this spec
//!
//! [Fast And Space Efficient Trie Searches]: https://infoscience.epfl.ch/record/64394/files/triesearches.pdf
//! [Ideal Hash Trees]: http://lampwww.epfl.ch/papers/idealhashtrees.pdf
//! [CHAMP paper]: https://michael.steindorfer.name/publications/oopsla15.pdf
//! [Java implementation]: https://github.com/msteindorfer/oopsla15-artifact/
//! [TrieMap_5Bits.java]: https://github.com/msteindorfer/oopsla15-artifact/blob/master/pdb.values/src/org/eclipse/imp/pdb/facts/util/TrieMap_5Bits.java
//! [Optimizing Hash-Array Mapped Tries for Fast and Lean Immutable JVM Collections]: https://blog.acolyer.org/2015/11/27/hamt/
//! [CHAMP Java]: https://github.com/Peergos/Peergos/blob/master/src/peergos/shared/hamt/Champ.java
//! [IAMap]: https://github.com/rvagg/iamap
//! [ipld-hashmap]: https://github.com/rvagg/js-ipld-hashmap
//! [go-hamt-ipld]: https://github.com/ipfs/go-hamt-ipld
//!
//! ## Summary
//!
//! The HAMT algorithm is used to build the IPLD HashMap.
//! This algorithm is common across many language standard libraries, particularly on the JVM (Clojure, Scala, Java),
//! to power very efficient in-memory unordered key/value storage data structures.
//! We extend the basic algorithm with buckets for block elasticity and strict mutation rules to ensure canonical form.
//!
//! The HAMT algorithm hashes incoming keys and uses incrementing subsections of
//! that hash at each level of its tree structure to determine the placement of
//! either the entry or a link to a child node of the tree.
//! A `bitWidth` determines the number of bits of the hash to use for index calculation at each level
//! of the tree such that the root node takes the first `bitWidth` bits of the hash to calculate an index
//! and as we move lower in the tree, we move along the hash by `depth x bitWidth` bits.
//! In this way, a sufficiently randomizing hash function will generate a hash that
//! provides a new index at each level of the data structure.
//! An index comprising `bitWidth` bits will generate index values of  `[ 0, 2`<sup>`bitWidth`</sup>` )`.
//! So a `bitWidth` of `8` will generate indexes of `0` to `255` inclusive.
//!
//! Each node in the tree can therefore hold up to `2`<sup>`bitWidth`</sup> elements of data,
//! which we store in an array. In the IPLD HashMap we store entries in buckets.
//! A `Set(key, value)` mutation where the index generated at the root node
//! for the hash of `key` denotes an array index that does not yet contain an entry,
//! we create a new bucket and insert the `key` / `value` pair entry.
//! In this way, a single node can theoretically hold up to `2`<sup>`bitWidth`</sup>` x bucketSize` entries,
//! where `bucketSize` is the maximum number of elements a bucket is allowed to contain ("collisions").
//! In practice, indexes do not distribute with perfect randomness so this maximum is theoretical.
//! Entries stored in the node's buckets are stored in `key`-sorted order.
//!
//! If a `Set(key, value)` mutation places a new entry in a bucket that already contains `bucketSize` entries,
//! we overflow to a new child node.
//! A new empty node is created and all existing entries in the bucket,
//! in addition to the new `key` / `value` pair entry are inserted into this new node.
//! We increment the `depth` for calculation of the `index` from each `key`'s hash value
//! to calculate the position in the new node's data array.
//! By incrementing `depth` we move along by `bitWidth` bits in each `key`'s hash.
//! With a sufficiently random hash function each `key` that generated the same `index`
//! at a previous level should be distributed roughly evenly in the new node's data array,
//! resulting in a node that contains up to `bucketSize` new buckets.
//!
//! The process of generating `index` values from `bitWidth` subsections of the hash values
//! provides us with a depth of up to `(digestLength x 8) / bitWidth` levels in our tree data structure
//! where `digestLength` is the number of output bytes generated by the hash function.
//! With each node able to store up to `2`<sup>`bitWidth`</sup> child node references
//! and up to `bucketSize` elements able to be stored in colliding leaf positions
//! we are able to store a very large number of entries.
//! A hash function's randomness will dictate the even distribution of elements
//! and a hash function's output `digestLength` will dictate the maximum depth of the tree.
//!
//! A further optimization is applied to reduce the storage requirements of HAMT nodes.
//! The data elements array is only allocated to be long enough to store actual entries:
//! non-empty buckets or links to actual child nodes.
//! An empty or `Null` array index is not used as a signal that a `key` does not exist in that node.
//! Instead, the data elements array is compacted by use of a `map` bitfield
//! where each bit of `map` corresponds to an `index` in the node.
//! When an `index` is generated, the `index` bit of the `map` bitfield is checked.
//! If the bit is not set (`0`), that index does not exist.
//! If the bit is set (`1`), the value exists in the data elements array.
//! To determine the index of the data elements array, we perform a bit-count (`popcount()`)
//! on the `map` bitfield _up to_ the `index` bit to generate a `dataIndex`.
//! In this way, the data elements array's total length is equal to `popcount(map)` (the number of bits set in all of `map`).
//! If `map`'s bits are all set then the data elements array will be `2`<sup>`bitWidth`</sup> in length,
//! i.e. every position will contain either a bucket or a link to a child node.
//!
//! Insertion of new buckets with `Set(key, value)` involves splicing in a new element
//! to the data array at the `dataIndex` position and setting the `index` bit of the `map` bitmap.
//! Converting a bucket to a child node leaves the `map` bit map alone as the `index` bit
//! still indicates there is an element at that position.
//!
//! A `Get(key)` operation performs the same hash, `index` and `dataIndex` calculation at the root node,
//! traversing into a bucket to find an entry matching `key`
//! or traversing into child nodes and performing the same `index` and `dataIndex` calculation
//! but at an offset of an additional `bitWidth` bits in the `key`'s hash.
//!
//! A `Delete(key)` mutation first locates the element in the same way as `Get(key)`
//! and if that entry exists, it is removed from the bucket containing it.
//! If the bucket is empty after deletion of the entry, we remove the bucket element
//! completely from the data element array and unsets the `index` bit of `map`.
//! If the node containing the deleted element has no links to child nodes
//! and contains `bucketSize` elements after the deletion,
//! those elements are compacted into a single bucket and placed in the parent node
//! in place of the link to that node.
//! We perform this check on the parent (and recursively if required),
//! thereby transforming the tree into its most compact form,
//! with only buckets in place of nodes that have up to `bucketSize` entries at all edges.
//! This compaction process combined with the `key` ordering of entries
//! in buckets produces canonical forms of the data structure for any given set of `key` / `value` pairs
//! regardless of their insertion order or whether any intermediate entries have been added and deleted.
//!
//! By default, each node in an IPLD HashMap is stored in a distinct IPLD block and CIDs are used for child node links.
//! The schema and algorithm presented here also allows for inline child nodes rather than links,
//! with read operations able to traverse multiple nodes within a single block where they are inlined.
//! The production of inlined IPLD HashMaps is left unspecified and users should be aware
//! that inlining breaks canonical form guarantees.
//!
//! ## Structure
//!
//! ### Parameters
//!
//! Configurable parameters for any given IPLD HashMap:
//!
//! * `hashAlg`: The hash algorithm applied to keys in order to evenly distribute entries throughout the data structure.
//! The algorithm is chosen based on speed, `digestLength` and randomness properties
//! (but it must be available to the reader, hence the need for shared defaults, see below).
//!
//! * `bitWidth`: The number of bits to use at each level of the data structure
//! for determining the index of the entry or a link to the next level of the data structure to continue searching.
//! The equation `2`<sup>`bitWidth`</sup> yields the arity of the HashMap nodes,
//! i.e. the number of storage locations for buckets and/or links to child nodes.
//!
//! * `bucketSize`: The maximum array size of entry storage buckets such that exceeding `bucketSize`
//! causes the creation of a new child node to replace entry storage.
//!
//! ### Node properties
//!
//! Each node in a HashMap data structure contains:
//!
//! * `data`: An Array, with a length of one to `2`<sup>`bitWidth`</sup>.
//! * `map`: A bitfield, stored as Bytes, where the first `2`<sup>`bitWidth`</sup> bits
//! are used to indicate whether a bucket or child node link is present at each possible index of the node.
//!
//! An important property of a HAMT is that the `data` array only contains active elements.
//! Indexes in a node that do not contain any values (in buckets or links to child nodes)
//! are not stored and the `map` bitfield is used to determine the `data` whether values are present
//! and the array index of present values using a [`popcount()`](https://en.wikipedia.org/wiki/Hamming_weight).
//! This allows us to store a maximally compacted `data` array for each node.
//!
//! ### Schema
//!
//! The **root block** of an IPLD HashMap contains the same properties as all other blocks,
//! in addition to configuration data that dictates how the algorithm below traverses and mutates the data structure.
//!
//! See [IPLD Schemas](https://github.com/ipld/specs/tree/master/schemas) for a definition of this format.
//!
//! ```ipldsch
//! # Root node layout
//! type HashMapRoot struct {
//!   hashAlg String
//!   bucketSize Int
//!   map Bytes
//!   data [ Element ]
//! }
//!
//! # Non-root node layout
//! type HashMapNode struct {
//!   map Bytes
//!   data [ Element ]
//! }
//!
//! type Element union {
//!   | HashMapNode map
//!   | &HashMapNode link
//!   | Bucket list
//! } representation kinded
//!
//! type Bucket [ BucketEntry ]
//!
//! type BucketEntry struct {
//!   key Bytes
//!   value Value
//! } representation tuple
//!
//! type Value union {
//!   | Bool bool
//!   | String string
//!   | Bytes bytes
//!   | Int int
//!   | Float float
//!   | Map map
//!   | List list
//!   | Link link
//! } representation kinded
//! ```
//!
//! Notes:
//!
//! * `hashAlg` in the root block is a string identifier for a hash algorithm.
//! The identifier should correspond to a [multihash](https://github.com/multiformats/multihash) identifier
//! as found in the [multiformats table](https://github.com/multiformats/multicodec/blob/master/table.csv).
//!
//! * `bitWidth` in the root block must be at least `3`, making the minimum `map` size 1 byte.
//!
//! * `bitWidth` is not present in the root block as it is inferred from the size of the `map` byte array
//! with the equation `log2(byteLength(map) x 8)`, being the inverse of the `map` size equation `2`<sup>`bitWidth`</sup>` / 8`.
//!
//! * `bucketSize` in the root block must be at least `1`.
//!
//! * Keys are stored in `Byte` form.
//!
//! * `Element` is a kinded union that supports storing either a `Bucket` (as kind list),
//! a link to a child node (as kind link), or as an inline, non-linked child node (as kind map).
//!

#![deny(missing_docs)]

mod bitfield;
mod error;
// mod hamt;
mod hash;
mod hash_bits;
mod node;
mod pointer;
