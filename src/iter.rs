//! Tree traversal iterators and traversal helpers.
//!
//! This module contains the iterator implementations used by [`crate::RedBlackTree`].
//! It supports:
//!
//! - in-order traversal (`LNR`);
//! - reverse in-order traversal (`RNL`);
//! - pre-order traversal (`NLR`);
//! - post-order traversal (`LRN`);
//! - level-order traversal.
//!
//! The iterator types are publicly re-exported by the crate when their
//! corresponding features are enabled. Pointer-based traversal helpers remain
//! internal implementation details.

use std::ptr::NonNull;

use crate::node::{Link, Node};

/*

            4
          /   \
         2     9
        / \   //\
       1   3  7   10
             / \
            6   8

    Depth-First Search / Pre-order (NLR):     4, 2, 1, 3, 9, 7, 6, 8, 10
    Depth-First Search / In-order (LNR):      1, 2, 3, 4, 6, 7, 8, 9, 10
    Depth-First Search / Post-order (LRN):    1, 3, 2, 6, 8, 7, 10, 9, 4
    Breadth-First Search / Level-order:       4, 2, 9, 1, 3, 7, 10, 6, 8

*/
#[cfg(feature = "in-order")]
pub use dfs_in_order::{IterInOrder, Range};

#[cfg(feature = "in-order")]
pub(crate) use dfs_in_order::{leftmost_child_in_order, successor_of_node_in_order};

#[cfg(feature = "in-order")]
mod dfs_in_order {
    use super::{Link, Node, NonNull};
    use std::iter::FusedIterator;
    use std::marker::PhantomData;

    /// Returns the next node in an in-order (`LNR`) traversal.
    ///
    /// # Safety
    ///
    /// `node` must point to a live node that belongs to the current tree, and
    /// all parent/child links reachable from it must be valid.
    pub unsafe fn successor_of_node_in_order<T>(node: NonNull<Node<T>>) -> Link<T> {
        unsafe {
            if node.as_ref().right.is_some() {
                // If a right subtree exists, the successor is its leftmost node.
                leftmost_child_in_order(node.as_ref().right)
            } else {
                // Otherwise, walk up through parent links until the current
                // node is found in a left subtree.
                parent_with_left(node)
            }
        }
    }

    /// Returns the leftmost node reachable from `node`.
    ///
    /// Returns `None` when `node` is `None`.
    ///
    /// # Safety
    ///
    /// Every pointer reachable through `left` links must refer to a live node.
    pub unsafe fn leftmost_child_in_order<T>(node: Link<T>) -> Link<T> {
        unsafe {
            if let Some(node) = node {
                if node.as_ref().left.is_none() {
                    Some(node)
                } else {
                    leftmost_child_in_order(node.as_ref().left)
                }
            } else {
                node
            }
        }
    }

    /// Walks up the parent chain until an ancestor is reached from its left child.
    ///
    /// # Safety
    ///
    /// `node` and all parent links traversed by this function must be valid.
    unsafe fn parent_with_left<T>(node: NonNull<Node<T>>) -> Link<T> {
        unsafe {
            let parent = node.as_ref().parent;
            if let Some(parent) = parent
                && let Some(left) = parent.as_ref().left
            {
                if std::ptr::eq(left.as_ptr(), node.as_ptr()) {
                    return Some(parent);
                }
                return parent_with_left(parent);
            }
            // No parent means that the root has been reached.
            None
        }
    }

    /// Iterator over a tree in in-order (`Left -> Node -> Right`, or `LNR`).
    ///
    /// For a binary search tree, this traversal yields values in ascending order.
    #[cfg(feature = "in-order")]
    pub struct IterInOrder<'a, T> {
        current_node: Link<T>,
        count: usize,
        elem: Option<&'a T>,
        _boo: PhantomData<&'a T>,
    }

    impl<'a, T> IterInOrder<'a, T> {
        /// Creates an in-order iterator starting at `node`.
        pub(crate) fn new(node: Link<T>, count: usize) -> Self {
            Self {
                current_node: node,
                count,
                elem: None,
                _boo: PhantomData,
            }
        }
    }

    impl<'a, T> Iterator for IterInOrder<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count > 0 {
                self.count -= 1;
                if let Some(node) = self.current_node {
                    unsafe {
                        self.elem = Some(&node.as_ref().value);
                        self.current_node = successor_of_node_in_order(node);
                    }
                } else {
                    self.elem = None;
                }
            } else {
                self.elem = None;
            }
            self.elem
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.count, Some(self.count))
        }
    }

    impl<'a, T> ExactSizeIterator for IterInOrder<'a, T> {
        fn len(&self) -> usize {
            self.count
        }
    }

    impl<'a, T> FusedIterator for IterInOrder<'a, T> {}

    /// A lazy iterator over an inclusive range of values in a [`crate::RedBlackTree`].
    ///
    /// `Range` is created by [`crate::RedBlackTree::range`]. It yields references to
    /// values in ascending order, starting from the first value greater than or
    /// equal to the lower bound and stopping after the upper bound.
    ///
    /// The iterator does not allocate an intermediate collection. Values are
    /// produced one at a time as the iterator advances.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbtree_extended::RedBlackTree;
    ///
    /// let mut tree = RedBlackTree::new();
    /// tree.extend([1, 3, 5, 7, 9, 11]);
    ///
    /// let values: Vec<_> = tree
    ///     .range(&3, &9)
    ///     .copied()
    ///     .collect();
    ///
    /// assert_eq!(values, vec![3, 5, 7, 9]);
    /// ```
    ///
    /// The iterator can be combined with standard iterator adapters:
    ///
    /// ```
    /// use rbtree_extended::RedBlackTree;
    ///
    /// let mut tree = RedBlackTree::new();
    /// tree.extend(1..=100);
    ///
    /// let values: Vec<_> = tree
    ///     .range(&20, &80)
    ///     .take(3)
    ///     .copied()
    ///     .collect();
    ///
    /// assert_eq!(values, vec![20, 21, 22]);
    /// ```
    pub struct Range<'a, T> {
        current_node: Link<T>,
        end_node: Link<T>,
        _marker: PhantomData<&'a T>,
    }

    impl<'a, T: Ord> Range<'a, T> {
        pub(crate) fn new(root: Link<T>, min: &T, max: &T) -> Self {
            if min > max {
                return Self {
                    current_node: None,
                    end_node: None,
                    _marker: PhantomData,
                };
            }

            unsafe {
                let mut current = root;
                let mut start: Link<T> = None;

                // First node >= min.
                while let Some(node) = current {
                    match node.as_ref().value.cmp(min) {
                        std::cmp::Ordering::Less => {
                            current = node.as_ref().right;
                        }

                        std::cmp::Ordering::Equal => {
                            start = Some(node);
                            break;
                        }

                        std::cmp::Ordering::Greater => {
                            start = Some(node);
                            current = node.as_ref().left;
                        }
                    }
                }

                current = root;

                let mut end: Link<T> = None;

                // First node > max.
                while let Some(node) = current {
                    match node.as_ref().value.cmp(max) {
                        std::cmp::Ordering::Less | std::cmp::Ordering::Equal => {
                            current = node.as_ref().right;
                        }

                        std::cmp::Ordering::Greater => {
                            end = Some(node);
                            current = node.as_ref().left;
                        }
                    }
                }

                Self {
                    current_node: start,
                    end_node: end,
                    _marker: PhantomData,
                }
            }
        }
    }

    impl<'a, T> Iterator for Range<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            let node = self.current_node?;

            if self.end_node == Some(node) {
                self.current_node = None;
                return None;
            }

            unsafe {
                let value = &node.as_ref().value;

                self.current_node = successor_of_node_in_order(node);

                Some(value)
            }
        }
    }
}

#[cfg(feature = "reverse-in-order")]
pub use dfs_reverse_in_order::IterReverseInOrder;

#[cfg(feature = "reverse-in-order")]
pub(crate) use dfs_reverse_in_order::{
    rightmost_child_reverse_in_order, successor_of_node_reverse_in_order,
};

#[cfg(feature = "reverse-in-order")]
mod dfs_reverse_in_order {
    use super::{Link, Node, NonNull};
    use std::marker::PhantomData;

    /// Returns the next node in reverse in-order (`Right -> Node -> Left`, or `RNL`).
    ///
    /// # Safety
    ///
    /// `node` must point to a live node that belongs to the current tree, and
    /// all parent/child links reachable from it must be valid.
    pub unsafe fn successor_of_node_reverse_in_order<T>(node: NonNull<Node<T>>) -> Link<T> {
        unsafe {
            if node.as_ref().left.is_some() {
                // After the current node, descend into the left subtree and
                // choose its rightmost node.
                rightmost_child_reverse_in_order(node.as_ref().left)
            } else {
                // Without a left subtree, walk upward until reaching an
                // ancestor from its right subtree.
                parent_with_right(node)
            }
        }
    }

    /// Returns the rightmost node reachable from `node`.
    ///
    /// Returns `None` when `node` is `None`.
    ///
    /// # Safety
    ///
    /// Every pointer reachable through `right` links must refer to a live node.
    pub unsafe fn rightmost_child_reverse_in_order<T>(node: Link<T>) -> Link<T> {
        unsafe {
            if let Some(node) = node {
                if node.as_ref().right.is_none() {
                    Some(node)
                } else {
                    rightmost_child_reverse_in_order(node.as_ref().right)
                }
            } else {
                None
            }
        }
    }

    /// Walks up the parent chain until an ancestor is reached from its right child.
    ///
    /// # Safety
    ///
    /// `node` and all parent links traversed by this function must be valid.
    unsafe fn parent_with_right<T>(node: NonNull<Node<T>>) -> Link<T> {
        unsafe {
            let parent = node.as_ref().parent;

            if let Some(parent) = parent
                && let Some(right) = parent.as_ref().right
            {
                if std::ptr::eq(right.as_ptr(), node.as_ptr()) {
                    return Some(parent);
                }

                return parent_with_right(parent);
            }

            None
        }
    }

    /// Iterator over a tree in reverse in-order (`Right -> Node -> Left`, or `RNL`).
    ///
    /// For a binary search tree, this traversal yields values in descending order.
    pub struct IterReverseInOrder<'a, T> {
        current_node: Link<T>,
        count: usize,
        elem: Option<&'a T>,
        _boo: PhantomData<&'a T>,
    }

    impl<'a, T> IterReverseInOrder<'a, T> {
        /// Creates a reverse in-order iterator starting at `node`.
        pub(crate) fn new(node: Link<T>, count: usize) -> Self {
            Self {
                current_node: node,
                count,
                elem: None,
                _boo: PhantomData,
            }
        }
    }

    impl<'a, T> Iterator for IterReverseInOrder<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count > 0 {
                self.count -= 1;

                if let Some(node) = self.current_node {
                    unsafe {
                        self.elem = Some(&node.as_ref().value);
                        self.current_node = successor_of_node_reverse_in_order(node);
                    }
                } else {
                    self.elem = None;
                }
            } else {
                self.elem = None;
            }

            self.elem
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.count, Some(self.count))
        }
    }
}

#[cfg(feature = "pre-order")]
pub use dfs_pre_order::IterPreOrder;

#[cfg(feature = "pre-order")]
pub(crate) use dfs_pre_order::successor_of_node_pre_order;

#[cfg(feature = "pre-order")]
mod dfs_pre_order {
    use super::{Link, Node, NonNull};
    use std::marker::PhantomData;

    /// Returns the next node in a pre-order (`Node -> Left -> Right`, or `NLR`) traversal.
    ///
    /// # Safety
    ///
    /// `node` must point to a live node in the current tree and all traversed
    /// links must be valid.
    pub unsafe fn successor_of_node_pre_order<T>(node: NonNull<Node<T>>) -> Link<T> {
        unsafe {
            if let Some(node) = node.as_ref().left {
                Some(node)
            } else if let Some(node) = node.as_ref().right {
                Some(node)
            } else {
                right_with_parent(node)
            }
        }
    }

    /// Finds the next right subtree by walking through parent links.
    ///
    /// # Safety
    ///
    /// `node` and all parent links traversed by this function must be valid.
    unsafe fn right_with_parent<T>(node: NonNull<Node<T>>) -> Link<T> {
        unsafe {
            if let Some(parent) = node.as_ref().parent {
                next_right(Some(parent), node)
            } else {
                None
            }
        }
    }

    /// Searches ancestors for the next unvisited right child.
    ///
    /// # Safety
    ///
    /// `child`, `node`, and all links traversed by this function must be valid.
    unsafe fn next_right<T>(node: Link<T>, child: NonNull<Node<T>>) -> Link<T> {
        unsafe {
            if let Some(n) = node {
                if let Some(right) = n.as_ref().right {
                    if std::ptr::eq(right.as_ptr(), child.as_ptr()) {
                        next_right(n.as_ref().parent, n)
                    } else {
                        Some(right)
                    }
                } else {
                    next_right(n.as_ref().parent, n)
                }
            } else {
                None
            }
        }
    }

    /// Iterator over a tree in pre-order (`Node -> Left -> Right`, or `NLR`).
    ///
    /// Pre-order traversal visits a node before either of its subtrees.
    pub struct IterPreOrder<'a, T> {
        current_node: Link<T>,
        count: usize,
        elem: Option<&'a T>,
        _boo: PhantomData<&'a T>,
    }

    impl<'a, T> IterPreOrder<'a, T> {
        /// Creates a pre-order iterator starting at `root`.
        pub(crate) fn new(root: Link<T>, count: usize) -> Self {
            Self {
                current_node: root,
                count,
                elem: None,
                _boo: PhantomData,
            }
        }
    }

    impl<'a, T> Iterator for IterPreOrder<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count > 0 {
                self.count -= 1;
                if let Some(node) = self.current_node {
                    unsafe {
                        self.elem = Some(&node.as_ref().value);
                        self.current_node = successor_of_node_pre_order(node);
                    }
                } else {
                    self.elem = None;
                }
            } else {
                self.elem = None;
            }
            self.elem
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.count, Some(self.count))
        }
    }
}

#[cfg(feature = "post-order")]
pub use dfs_post_order::IterPostOrder;

#[cfg(feature = "post-order")]
pub(crate) use dfs_post_order::{leaf_post_order, successor_of_node_post_order};

#[cfg(feature = "post-order")]
mod dfs_post_order {
    use super::{Link, Node, NonNull};
    use std::marker::PhantomData;

    /// Returns the next node in a post-order (`Left -> Right -> Node`, or `LRN`) traversal.
    ///
    /// # Safety
    ///
    /// `current_node` must point to a live node in the current tree and all
    /// traversed parent/child links must be valid.
    pub unsafe fn successor_of_node_post_order<T>(current_node: NonNull<Node<T>>) -> Link<T> {
        unsafe {
            if let Some(parent) = current_node.as_ref().parent {
                if let Some(right) = parent.as_ref().right {
                    if std::ptr::eq(current_node.as_ptr(), right.as_ptr()) {
                        Some(parent)
                    } else {
                        Some(leaf_post_order(right))
                    }
                } else {
                    Some(parent)
                }
            } else {
                None
            }
        }
    }

    /// Returns the first leaf visited by a post-order traversal starting at `node`.
    ///
    /// # Safety
    ///
    /// `node` must point to a live node and all descendant links traversed by
    /// this function must be valid.
    pub unsafe fn leaf_post_order<T>(node: NonNull<Node<T>>) -> NonNull<Node<T>> {
        unsafe {
            if let Some(left) = node.as_ref().left {
                leaf_post_order(left)
            } else if let Some(right) = node.as_ref().right {
                leaf_post_order(right)
            } else {
                node
            }
        }
    }

    /// Iterator over a tree in post-order (`Left -> Right -> Node`, or `LRN`).
    ///
    /// Post-order traversal visits both subtrees before the current node.
    pub struct IterPostOrder<'a, T> {
        current_node: Link<T>,
        count: usize,
        _boo: PhantomData<&'a T>,
    }

    impl<'a, T> IterPostOrder<'a, T> {
        /// Creates a post-order iterator starting at the first post-order node.
        pub(crate) fn new(root: Link<T>, count: usize) -> Self {
            Self {
                current_node: root,
                count,
                _boo: PhantomData,
            }
        }
    }

    impl<'a, T> Iterator for IterPostOrder<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count == 0 {
                return None;
            }

            let node = self.current_node?;

            unsafe {
                let value = &node.as_ref().value;
                self.current_node = successor_of_node_post_order(node);
                self.count -= 1;
                Some(value)
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.count, Some(self.count))
        }
    }
}

/// Breadth-First Search
#[cfg(feature = "bfs")]
mod bfs {
    use super::{Node, NonNull};
    use std::collections::VecDeque;

    /// Appends tree values to `ret` in breadth-first / level-order.
    pub fn iter_level_order_impl<T>(root: NonNull<Node<T>>, ret: &mut Vec<&T>) {
        let mut deque: VecDeque<NonNull<Node<T>>> = VecDeque::new();
        unsafe {
            deque.push_back(root);
            let mut node: NonNull<Node<T>>;
            while !deque.is_empty() {
                node = deque.pop_front().unwrap();
                ret.push(&node.as_ref().value);
                if let Some(left) = node.as_ref().left {
                    deque.push_back(left);
                }
                if let Some(right) = node.as_ref().right {
                    deque.push_back(right);
                }
            }
        }
    }
}

#[cfg(feature = "bfs")]
pub(crate) use bfs::iter_level_order_impl;
