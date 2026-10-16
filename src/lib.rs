//! # rbtree_extended
//!
//! `rbtree_extended` is a generic Red-Black Tree implementation for Rust.
//!
//! A Red-Black Tree is a self-balancing binary search tree.
//! It keeps values ordered while automatically rebalancing itself after
//! insertions and removals.
//!
//! In practice, this means that common operations such as search, insertion,
//! and removal remain efficient even when values are inserted in an
//! unfavorable order.
//!
//! ## When is a Red-Black Tree useful?
//!
//! <details>
//! <summary>A Red-Black Tree is useful when you need:</summary>
//!
//! - values to remain sorted;
//! - fast lookup, insertion, and removal;
//! - ordered iteration;
//! - minimum and maximum values;
//! - predecessor and successor navigation;
//! - lower and upper bound queries;
//! - range queries;
//! - several tree traversal strategies;
//! - visualization of the tree structure.
//!
//! Unlike a simple binary search tree, a Red-Black Tree automatically keeps
//! its height under control by performing rotations and recoloring.
//!
//! For example, inserting already sorted values into a normal binary search
//! tree may produce a structure similar to:
//!
//! ```text
//! 1
//!  \
//!   2
//!    \
//!     3
//!      \
//!       4
//! ```
//!
//! Such a tree behaves almost like a linked list.
//!
//! A Red-Black Tree automatically rebalances itself, keeping the structure
//! much closer to:
//!
//! ```text
//!       2
//!      / \
//!     1   4
//!        /
//!       3
//! ```
//!
//! This keeps the height logarithmic and allows lookup, insertion, and removal
//! to remain efficient.
//! </details>
//!
//! ## Implementation approach in Rust
//!
//! Implementing a tree in Rust is more interesting than implementing the same
//! data structure in languages with unrestricted shared mutable pointers.
//!
//! <details>
//! <summary>Read more...</summary>
//!
//! A tree node naturally wants to reference other nodes:
//!
//! ```text
//!        parent
//!          ↑
//!          │
//!        [node]
//!        /    \
//!     left    right
//! ```
//!
//! For a Red-Black Tree, parent links are especially useful because insertion
//! and removal balancing frequently need to move upward through the tree and
//! perform rotations.
//!
//! There are several common ways to represent such structures in Rust.
//!
//! ### `Using Box<Node<T>>`
//!
//! A simple owned binary tree can use:
//!
//! ```text
//! Option<Box<Node<T>>>
//! ```
//!
//! This works very well when ownership always flows from parent to children.
//!
//! However, once nodes also need parent links, the ownership graph is no
//! longer purely hierarchical:
//!
//! ```text
//! parent owns child
//!
//! parent <----- child
//!   |             |
//!   +------------>+
//!       parent
//!       pointer
//! ```
//!
//! A child cannot simply own its parent as well, because that would create
//! cyclic ownership.
//!
//! ### `Using Rc<RefCell<Node<T>>>`
//!
//! Another common approach is to combine shared ownership and interior
//! mutability:
//!
//! ```text
//! Rc<RefCell<Node<T>>>
//! ```
//!
//! and usually `Weak` for parent links.
//!
//! This is a fully safe approach and can be convenient for graph-like
//! structures, but it introduces additional machinery:
//!
//! - reference counting;
//! - runtime borrow checking;
//! - `Rc`, `Weak`, and `RefCell` wrappers around node access;
//! - more indirect code during rotations and rebalancing.
//!
//! ### Using Arena or index-based storage
//!
//! Nodes can also be stored in a separate container and reference each other
//! using indices or handles:
//!
//! ```text
//! Vec<Node<T>>
//!
//! left:   Option<usize>
//! right:  Option<usize>
//! parent: Option<usize>
//! ```
//!
//! This avoids direct raw pointers and can be a very good design, especially
//! when stable handles or compact storage are important.
//!
//! It also changes the nature of the implementation: tree links become indices
//! into an external storage arena rather than direct links between nodes.
//!
//! ### Why this crate uses `Option<NonNull<Node<T>>>`
//!
//! This implementation uses direct non-owning node links:
//!
//! ```text
//! type Link<T> = Option<NonNull<Node<T>>>;
//! ```
//!
//! Each node is allocated on the heap and the links between nodes are stored as
//! non-null raw pointers wrapped in `Option`:
//!
//! ```text
//! struct Node<T> {
//!     left: Link<T>,
//!     right: Link<T>,
//!     parent: Link<T>,
//!     is_red: bool,
//!     value: T,
//! }
//! ```
//!
//! This representation was chosen because it maps closely to the traditional
//! pointer-based description of a Red-Black Tree.
//!
//! It gives the implementation:
//!
//! - direct parent, left, and right links;
//! - stable node addresses while nodes remain allocated;
//! - straightforward tree rotations;
//! - efficient upward and downward navigation;
//! - no reference-counting overhead;
//! - no runtime `RefCell` borrow checking.
//!
//! `NonNull<Node<T>>` expresses that an existing link always points to a valid
//! non-null node, while `Option` represents the absence of a link:
//!
//! ```text
//! None       -> no child / no parent
//! Some(ptr)  -> existing node
//! ```
//!
//! `Option<NonNull<T>>` also benefits from Rust's nullable-pointer
//! representation optimization, so the optional link can be represented with
//! the same size as a pointer.
//!
//! The trade-off is important: `NonNull` does not make pointer manipulation
//! safe by itself.
//!
//! Tree operations must manually preserve invariants such as:
//!
//! - every live pointer refers to an allocated node;
//! - no pointer is used after the node is freed;
//! - parent and child links remain synchronized;
//! - rotations update every affected link;
//! - each allocation is released exactly once;
//! - references returned to users never outlive the tree.
//!
//! Because of this, the internal implementation contains `unsafe` code, while
//! the public API remains safe.
//!
//! This design is intentionally low-level: one of the goals of the crate is to
//! demonstrate how a pointer-based self-balancing tree can be implemented in
//! Rust while keeping the unsafe surface internal.
//!
//!
//! ### Further reading
//!
//! A particularly useful resource for understanding linked data structures,
//! ownership, raw pointers, and progressively more advanced implementations in
//! Rust is:
//!
//! * [Learn Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/)
//!
//! The book explores several representations of linked structures in Rust,
//! starting with simple ownership-based implementations and eventually moving
//! into raw-pointer-based designs. It is a useful companion for understanding
//! why data structures such as this one may choose `NonNull` internally.
//!
//! </details>
//!
//! ## Implementation details
//!
//! Internally, the tree uses `Option<NonNull<Node<T>>>` for parent and child
//! links. This allows rotations and bidirectional navigation without reference
//! counting or interior mutability.
//!
//! The implementation therefore contains `unsafe` code internally. The public
//! API is designed to remain safe: users do not need to write `unsafe` code
//! when working with `RedBlackTree`.
//!
//! The unsafe implementation relies on internal invariants that keep node
//! pointers valid and maintain correct ownership of allocated nodes.
//!
//! ## Basic usage
//!
//! ```
//! use rbtree_extended::RedBlackTree;
//!
//! let mut tree = RedBlackTree::new();
//!
//! tree.insert(8);
//! tree.insert(3);
//! tree.insert(10);
//! tree.insert(1);
//! tree.insert(6);
//!
//! assert_eq!(tree.len(), 5);
//! assert!(tree.contains(&6));
//! assert!(!tree.contains(&100));
//!
//! assert_eq!(tree.first(), Some(&1));
//! assert_eq!(tree.last(), Some(&10));
//! ```
//!
//! Values are unique. Inserting the same value twice returns `false`:
//!
//! ```
//! use rbtree_extended::RedBlackTree;
//!
//! let mut tree = RedBlackTree::new();
//!
//! assert!(tree.insert(10));
//! assert!(!tree.insert(10));
//! ```
//!
//! ## Ordered iteration
//!
//! The default iterator is an in-order traversal.
//!
//!
//! ```
//! use rbtree_extended::RedBlackTree;
//!
//! let mut tree = RedBlackTree::new();
//!
//! for value in [8, 3, 10, 1, 6, 14, 4, 7, 13] {
//!     tree.insert(value);
//! }
//!
//! let values: Vec<i32> = tree.iter().copied().collect();
//!
//! assert_eq!(
//!     values,
//!     vec![1, 3, 4, 6, 7, 8, 10, 13, 14]
//! );
//! ```
//!
//! ## Tree traversals
//!
//! The crate supports several traversal strategies.
//!
//! Consider this tree:
//!
//! ```text
//!         4
//!       /   \
//!      2     9
//!     / \   / \
//!    1   3 7  10
//!         / \
//!        6   8
//! ```
//!
//! The traversals visit nodes in different orders.
//!
//! ### In-order
//!
//! Left subtree → current node → right subtree.
//!
//! ```text
//! 1, 2, 3, 4, 6, 7, 8, 9, 10
//! ```
//!
//! <details>
//! <summary>Example:</summary>
//!
//! ```
//! use rbtree_extended::RedBlackTree;
//!
//! let mut tree = RedBlackTree::new();
//!
//! for value in [4, 3, 9, 1, 2, 10, 7, 8, 6] {
//!     tree.insert(value);
//! }
//!
//! let values: Vec<i32> = tree.iter_inorder().copied().collect();
//!
//! assert_eq!(
//!     values,
//!     vec![1, 2, 3, 4, 6, 7, 8, 9, 10]
//! );
//! ```
//! </details>
//!
//! ### Reverse in-order
//!
//! Right subtree → current node → left subtree.
//!
//! ```text
//! 10, 9, 8, 7, 6, 4, 3, 2, 1
//! ```
//!
//! <details>
//! <summary>Example:</summary>
//!
//! For a binary search tree, reverse in-order traversal returns values in
//! descending order:
//!
//! ```
//! use rbtree_extended::RedBlackTree;
//!
//! let mut tree = RedBlackTree::new();
//!
//! for value in [4, 3, 9, 1, 2, 10, 7, 8, 6] {
//!     tree.insert(value);
//! }
//!
//! let values: Vec<i32> = tree.iter_reverse_inorder().copied().collect();
//!
//! assert_eq!(
//!     values,
//!     vec![10, 9, 8, 7, 6, 4, 3, 2, 1]
//! );
//! ```
//! </details>
//!
//!
//! ### Pre-order
//!
//! Current node → left subtree → right subtree.
//!
//! ```text
//! 4, 2, 1, 3, 9, 7, 6, 8, 10
//! ```
//!
//! <details>
//! <summary>Example:</summary>
//!
//! Pre-order traversal is useful when the structure of the tree itself matters,
//! for example when serializing or inspecting it.
//!
//! ```
//! use rbtree_extended::RedBlackTree;
//!
//! let mut tree = RedBlackTree::new();
//!
//! for value in [4, 3, 9, 1, 2, 10, 7, 8, 6] {
//!     tree.insert(value);
//! }
//!
//! let values: Vec<i32> = tree.iter_preorder().copied().collect();
//!
//! assert_eq!(
//!     values,
//!     vec![4, 2, 1, 3, 9, 7, 6, 8, 10]
//! );
//! ```
//!
//! </details>
//!
//!
//! ### Post-order
//!
//! Left subtree → right subtree → current node.
//!
//! ```text
//! 1, 3, 2, 6, 8, 7, 10, 9, 4
//! ```
//!
//! <details>
//! <summary>Example:</summary>
//!
//! Post-order traversal is often useful when child nodes need to be processed
//! before their parent.
//!
//! ```
//! use rbtree_extended::RedBlackTree;
//!
//! let mut tree = RedBlackTree::new();
//!
//! for value in [4, 3, 9, 1, 2, 10, 7, 8, 6] {
//!     tree.insert(value);
//! }
//!
//! let values: Vec<i32> = tree.iter_postorder().copied().collect();
//!
//! assert_eq!(
//!     values,
//!     vec![1, 3, 2, 6, 8, 7, 10, 9, 4]
//! );
//! ```
//!
//! </details>
//!
//!
//! ### Level-order
//!
//! Breadth-first traversal visits the tree level by level:
//!
//! ```text
//! 4, 2, 9, 1, 3, 7, 10, 6, 8
//! ```
//!
//! <details>
//! <summary>Example:</summary>
//!
//! ```
//! use rbtree_extended::RedBlackTree;
//!
//! let mut tree = RedBlackTree::new();
//!
//! for value in [4, 3, 9, 1, 2, 10, 7, 8, 6] {
//!     tree.insert(value);
//! }
//!
//! let values: Vec<i32> = tree
//!     .iter_level_order()
//!     .into_iter()
//!     .copied()
//!     .collect();
//!
//! assert_eq!(
//!     values,
//!     vec![4, 2, 9, 1, 3, 7, 10, 6, 8]
//! );
//! ```
//!
//! </details>
//!
//! ## Successor and predecessor
//!
//! Because the tree is ordered, you can navigate to the next or previous value.
//!
//! ```
//! use rbtree_extended::RedBlackTree;
//!
//! let mut tree = RedBlackTree::new();
//!
//! for value in [1, 3, 4, 6, 7, 8, 10] {
//!     tree.insert(value);
//! }
//!
//! assert_eq!(tree.successor(&6), Some(&7));
//! assert_eq!(tree.predecessor(&6), Some(&4));
//!
//! assert_eq!(tree.successor(&10), None);
//! assert_eq!(tree.predecessor(&1), None);
//! ```
//!
//! The crate also provides successor operations for specific traversal orders:
//!
//! - [`RedBlackTree::successor_dfs_in_order`]
//! - [`RedBlackTree::successor_dfs_pre_order`]
//! - [`RedBlackTree::successor_dfs_post_order`]
//!
//! ## Lower and upper bounds
//!
//! `lower_bound` returns the first value greater than or equal to the requested
//! value.
//!
//! `upper_bound` returns the first value strictly greater than the requested
//! value.
//!
//! ```
//! use rbtree_extended::RedBlackTree;
//!
//! let mut tree = RedBlackTree::new();
//!
//! for value in [1, 3, 6, 8, 10, 13] {
//!     tree.insert(value);
//! }
//!
//! assert_eq!(tree.lower_bound(&5), Some(&6));
//! assert_eq!(tree.lower_bound(&6), Some(&6));
//!
//! assert_eq!(tree.upper_bound(&6), Some(&8));
//! assert_eq!(tree.upper_bound(&13), None);
//! ```
//!
//! ## Range queries
//!
//! [`RedBlackTree::range`] returns a lazy iterator over an inclusive range.
//! Values are produced in sorted order without allocating an intermediate
//! collection.
//!
//! ```
//! use rbtree_extended::RedBlackTree;
//!
//! let mut tree = RedBlackTree::new();
//! tree.extend([1, 3, 5, 7, 9, 11]);
//!
//! let values: Vec<_> = tree
//!     .range(&3, &9)
//!     .copied()
//!     .collect();
//!
//! assert_eq!(values, vec![3, 5, 7, 9]);
//!
//! // Because the range is lazy, iterator adapters work naturally.
//! let first_two: Vec<_> = tree
//!     .range(&3, &11)
//!     .take(2)
//!     .copied()
//!     .collect();
//!
//! assert_eq!(first_two, vec![3, 5]);
//! ```
//!
//! ## Filtering values with `retain`
//!
//! [`RedBlackTree::retain`] keeps only the values that satisfy a predicate.
//! Every value for which the predicate returns `false` is removed from the tree.
//!
//! This is useful when the tree already contains ordered data and only a subset
//! of the values should remain.
//!
//! ```
//! use rbtree_extended::RedBlackTree;
//!
//! let mut tree = RedBlackTree::new();
//! tree.extend([1, 2, 3, 4, 5, 6]);
//!
//! // Keep only even values.
//! tree.retain(|value| value % 2 == 0);
//!
//! assert_eq!(
//!     tree.iter().copied().collect::<Vec<_>>(),
//!     vec![2, 4, 6]
//! );
//! ```
//!
//! The predicate can also use fields of custom types:
//!
//! ```
//! use std::cmp::Ordering;
//! use rbtree_extended::RedBlackTree;
//!
//! #[derive(Debug, Default)]
//! struct Task {
//!     id: u32,
//!     priority: u32,
//! }
//!
//! impl PartialEq for Task {
//!     fn eq(&self, other: &Self) -> bool {
//!         self.id == other.id
//!     }
//! }
//!
//! impl Eq for Task {}
//!
//! impl PartialOrd for Task {
//!     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//!         Some(self.cmp(other))
//!     }
//! }
//!
//! impl Ord for Task {
//!     fn cmp(&self, other: &Self) -> Ordering {
//!         self.id.cmp(&other.id)
//!     }
//! }
//!
//! let mut tasks = RedBlackTree::new();
//!
//! tasks.insert(Task { id: 1, priority: 10 });
//! tasks.insert(Task { id: 2, priority: 100 });
//! tasks.insert(Task { id: 3, priority: 50 });
//!
//! tasks.retain(|task| task.priority >= 50);
//!
//! let ids: Vec<u32> = tasks.iter().map(|task| task.id).collect();
//!
//! assert_eq!(ids, vec![2, 3]);
//! ```
//!
//! ## Removal
//!
//! Values can be removed while preserving Red-Black Tree invariants.
//!
//! ```
//! use rbtree_extended::RedBlackTree;
//!
//! let mut tree = RedBlackTree::new();
//!
//! for value in [10, 5, 20] {
//!     tree.insert(value);
//! }
//!
//! assert!(tree.remove(&10));
//! assert!(!tree.contains(&10));
//! assert!(!tree.remove(&100));
//! ```
//!
//! ## Graphviz DOT export
//!
//! A tree can be exported to the
//! [Graphviz DOT](https://graphviz.org/doc/info/lang.html) format.
//!
//! This is useful for inspecting the real internal structure of the tree after
//! balancing.
//!
//!
//! ```
//! use rbtree_extended::RedBlackTree;
//!
//! let mut tree = RedBlackTree::new();
//!
//! for value in [8, 3, 10, 1, 6, 14, 4, 7, 13] {
//!     tree.insert(value);
//! }
//!
//! let dot = tree.to_dot();
//!
//! println!("{dot}");
//! ```
//!
//! The resulting string can be rendered by Graphviz or pasted into an [online
//! DOT viewer](https://dreampuf.github.io/GraphvizOnline/).
//!
//! ![Red-Black Tree example](https://github.com/Jekahome/rbtree-extended/blob/main/docs/rbtree-example.svg)
//!
//!
//! A generated graph contains both the tree connections and the Red-Black
//! colors of the nodes.
//!
//! An empty tree also produces a valid DOT graph.
//!
//! ## Custom types
//!
//! `RedBlackTree` is generic and can store custom values as long as they
//! implement the required ordering traits.
//!
//! The ordering defined by `Ord` determines how values are positioned in the
//! tree.
//!
//! ```
//! use std::cmp::Ordering;
//! use rbtree_extended::RedBlackTree;
//!
//! #[derive(Debug, Default)]
//! struct User {
//!     id: u32,
//!     name: String,
//! }
//!
//! // Users are ordered only by `id`.
//! // The `name` field does not participate in comparisons.
//! impl PartialEq for User {
//!     fn eq(&self, other: &Self) -> bool {
//!         self.id == other.id
//!     }
//! }
//!
//! impl Eq for User {}
//!
//! impl PartialOrd for User {
//!     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//!         Some(self.cmp(other))
//!     }
//! }
//!
//! impl Ord for User {
//!     fn cmp(&self, other: &Self) -> Ordering {
//!         self.id.cmp(&other.id)
//!     }
//! }
//!
//! let mut users = RedBlackTree::new();
//!
//! users.insert(User {
//!     id: 2,
//!     name: "Alice".into(),
//! });
//!
//! users.insert(User {
//!     id: 1,
//!     name: "Bob".into(),
//! });
//!
//! users.insert(User {
//!     id: 3,
//!     name: "Charlie".into(),
//! });
//!
//! assert_eq!(users.first().unwrap().id, 1);
//! assert_eq!(users.last().unwrap().id, 3);
//!
//! // A lookup key only needs the fields used by `Ord`.
//! let key = User {
//!     id: 2,
//!     name: String::new(),
//! };
//!
//! // `get` returns a reference to the stored value.
//! let user = users.get(&key).unwrap();
//! assert_eq!(user.name, "Alice");
//!
//! // `take` removes the matching value and returns ownership of it.
//! let removed = users.take(&key).unwrap();
//!
//! assert_eq!(removed.id, 2);
//! assert_eq!(removed.name, "Alice");
//! assert!(!users.contains(&key));
//! assert_eq!(users.len(), 2);
//! ```
//!
//! ## Complexity
//!
//! For a tree containing `n` values:
//!
//! | Operation | Complexity |
//! |-----------|------------|
//! | Search | `O(log n)` |
//! | Insert | `O(log n)` |
//! | Remove | `O(log n)` |
//! | Minimum / maximum | `O(log n)` |
//! | Successor / predecessor | `O(log n)` |
//! | Lower / upper bound | `O(log n)` |
//! | Full traversal | `O(n)` |
//! | Range query | `O(log n + k)` |
//! | Retain    | `O(n log n)`  |
//!
//! where `k` is the number of values returned by the range query.
//!
//! ## Main API
//!
//! The main type provided by this crate is [`RedBlackTree`].
//!
//! Common methods include:
//!
//! - [`RedBlackTree::insert`]
//! - [`RedBlackTree::remove`]
//! - [`RedBlackTree::replace`]
//! - [`RedBlackTree::take`]
//! - [`RedBlackTree::retain`]
//! - [`RedBlackTree::pop_first`]
//! - [`RedBlackTree::pop_last`]
//! - [`RedBlackTree::get`]
//! - [`RedBlackTree::contains`]
//! - [`RedBlackTree::first`]
//! - [`RedBlackTree::last`]
//! - [`RedBlackTree::successor`]
//! - [`RedBlackTree::predecessor`]
//! - [`RedBlackTree::lower_bound`]
//! - [`RedBlackTree::upper_bound`]
//! - [`RedBlackTree::range`]
//! - [`RedBlackTree::iter`]
//! - [`RedBlackTree::iter_inorder`]
//! - [`RedBlackTree::iter_reverse_inorder`]
//! - [`RedBlackTree::iter_preorder`]
//! - [`RedBlackTree::iter_postorder`]
//! - [`RedBlackTree::iter_level_order`]
//! - [`RedBlackTree::to_dot`]
//!
//! See the individual method documentation for detailed examples.

#![forbid(unsafe_op_in_unsafe_fn)]

mod iter;
mod node;
mod tree;
mod validation;

pub use tree::RedBlackTree;

#[cfg(feature = "in-order")]
pub use iter::{IterInOrder, Range};

#[cfg(feature = "pre-order")]
pub use iter::IterPreOrder;

#[cfg(feature = "reverse-in-order")]
pub use iter::IterReverseInOrder;

#[cfg(feature = "post-order")]
pub use iter::IterPostOrder;
