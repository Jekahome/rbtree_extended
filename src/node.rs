//! Internal node representation used by the Red-Black Tree.
//!
//! Nodes are allocated on the heap and connected using [`NonNull`] pointers.
//! The tree owns the allocated nodes and is responsible for releasing them.
//!
//! `Node` and [`Link`] are crate-private implementation details and are not
//! exposed through the public API.

use std::ptr::NonNull;

/// A single node in the Red-Black Tree.
///
/// Each node stores:
///
/// - a link to its left child;
/// - a link to its right child;
/// - a link to its parent;
/// - its Red-Black color;
/// - the stored value.
///
/// Tree links are represented by [`Link<T>`], which is an
/// `Option<NonNull<Node<T>>>`.
///
/// A `None` link represents the absence of a parent or child, while
/// `Some(pointer)` refers to another allocated node.
///
/// Memory ownership is managed by [`crate::RedBlackTree`]. The pointers stored
/// here are non-owning links between nodes.
#[derive(Debug)]
pub(crate) struct Node<T> {
    /// Left child of this node.
    pub(crate) left: Link<T>,

    /// Right child of this node.
    pub(crate) right: Link<T>,

    /// Parent of this node.
    ///
    /// The root node has no parent.
    pub(crate) parent: Link<T>,

    /// Red-Black color of the node.
    ///
    /// `true` means red and `false` means black.
    pub(crate) is_red: bool,

    /// Value stored in this node.
    pub(crate) value: T,
}

/// A link between nodes in the tree.
///
/// `None` represents a missing parent or child.
///
/// `Some(ptr)` contains a non-null pointer to another allocated [`Node`].
///
/// Using `Option<NonNull<Node<T>>>` allows the implementation to represent
/// nullable tree links while keeping direct pointer-based navigation between
/// parents and children.
pub(crate) type Link<T> = Option<NonNull<Node<T>>>;

impl<T: Default> Node<T> {
    /// Creates the sentinel head node together with the initial tree root.
    ///
    /// The sentinel does not represent a value stored by the user. Its
    /// `value` field is initialized with [`Default::default`] and its left
    /// child becomes the first actual node in the tree.
    ///
    /// The initial root node is created as black, as required by the
    /// Red-Black Tree invariants.
    ///
    /// # Returns
    ///
    /// A link to the newly allocated sentinel node.
    ///
    /// # Implementation
    ///
    /// Both the sentinel and the root node are allocated on the heap.
    /// Ownership of these allocations is transferred to the tree and later
    /// recovered when the tree releases its nodes.
    pub(crate) fn new_fixed_head(value: T) -> Link<T> {
        let fixed_head = Box::into_non_null(Box::new(Self {
            left: None,
            right: None,
            parent: None,
            is_red: false,
            value: T::default(),
        }));
        unsafe {
            (*fixed_head.as_ptr()).left = Node::new_black(value);
        }
        Some(fixed_head)
    }
}

impl<T> Node<T> {
    /// Allocates a new black node.
    ///
    /// The node is created without a parent or children.
    ///
    /// # Returns
    ///
    /// A non-null link to the newly allocated node.
    ///
    /// The allocation becomes owned by the tree and must eventually be
    /// released by the tree implementation.
    pub(crate) fn new_black(value: T) -> Link<T> {
        let new = Box::into_non_null(Box::new(Self {
            left: None,
            right: None,
            parent: None,
            is_red: false,
            value,
        }));

        Some(new)
    }

    /// Allocates a new red node with the specified parent.
    ///
    /// Newly inserted Red-Black Tree nodes are initially red. The balancing
    /// algorithm may later recolor the node or rotate the surrounding
    /// subtree.
    ///
    /// The new node has no children.
    ///
    /// # Parameters
    ///
    /// - `value` — value stored in the node.
    /// - `parent` — non-null pointer to the node's parent.
    ///
    /// # Returns
    ///
    /// A non-null link to the newly allocated node.
    ///
    /// The allocation becomes owned by the tree and must eventually be
    /// released by the tree implementation.
    pub(crate) fn new_red(value: T, parent: NonNull<Node<T>>) -> Link<T> {
        let new = Box::into_non_null(Box::new(Self {
            left: None,
            right: None,
            parent: Some(parent),
            is_red: true,
            value,
        }));

        Some(new)
    }
}
