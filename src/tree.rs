use std::cmp::Ordering;
use std::fmt::{Debug, Display};

use crate::node::{Link, Node};

use std::marker::PhantomData;
use std::ptr::NonNull;

#[cfg(feature = "in-order")]
use crate::iter::{IterInOrder, Range, leftmost_child_in_order, successor_of_node_in_order};

#[cfg(feature = "reverse-in-order")]
use crate::iter::{
    IterReverseInOrder, rightmost_child_reverse_in_order, successor_of_node_reverse_in_order,
};

#[cfg(feature = "pre-order")]
use crate::iter::{IterPreOrder, successor_of_node_pre_order};

#[cfg(feature = "post-order")]
use crate::iter::{IterPostOrder, leaf_post_order, successor_of_node_post_order};

#[cfg(feature = "bfs")]
use crate::iter::iter_level_order_impl;

/// A self-balancing Red-Black Tree.
///
/// `RedBlackTree` stores unique values in sorted order.
///
/// Insert, lookup and removal operations have logarithmic
/// expected tree height due to Red-Black Tree balancing.
///
/// # Examples
///
/// ```
/// use rbtree_extended::RedBlackTree;
///
/// let mut tree = RedBlackTree::new();
///
/// tree.insert(3);
/// tree.insert(1);
/// tree.insert(2);
///
/// assert_eq!(tree.len(), 3);
/// assert!(tree.contains(&2));
/// ```
pub struct RedBlackTree<T: Ord + Default> {
    pub(crate) fixed_head: Link<T>,
    pub(crate) count: usize,
    _marker: PhantomData<T>,
}

enum OperationPut {
    Left,
    Right,
    FlipColors,
    Nothing,
}

enum OperationRemove {
    RedLeaf,
    BlackLeaf,
    NodeWithChildren,
    BlackNodeWithRedLeaf,
}

enum OperationRemoveBlackLeaf {
    LeftRedABlackBRedCleaf,                      // 2.1.1.1
    RightRedABlackBRedCleaf,                     // 2.1.1.2
    LeftRedABlackBleaf,                          // 2.1.2.1
    RightRedABlackBleaf,                         // 2.1.2.2
    BlackARedBWithBlackChildrenLeaf,             // 2.2.4
    BlackARedBWithBlackChildrenRightHaveRedLeaf, // 2.2.1
    BlackALeftBlackBRedDleaf,                    // 2.3.1.1
    BlackARightBlackBRedDleaf,                   // 2.3.1.2
    BlackALeftBlackBleaf,                        // 2.3.2.1
    BlackARightBlackBleaf,                       // 2.3.2.2
    Root,
    Unimplemented,
}

impl<T: Ord + Default + Display + Debug> RedBlackTree<T> {
    #[cfg(test)]
    pub fn check_connections(&self) {
        unsafe {
            let fixed_head = self.fixed_head.unwrap();
            let node = (*fixed_head.as_ptr()).left;
            return checking_connections(node);
        }

        unsafe fn checking_connections<T: Ord + Default + Display + Debug>(node: Link<T>) {
            unsafe {
                if let Some(n) = node {
                    if (*n.as_ptr()).parent.is_none() {
                        println!("\nROOT:[{}]", (*n.as_ptr()).value);
                    }
                    if (*n.as_ptr()).left.is_some() && (*n.as_ptr()).right.is_some() {
                        let left = (*n.as_ptr()).left.unwrap();
                        let right = (*n.as_ptr()).right.unwrap();
                        println!(
                            "[{}] <- [{}] -> [{}]",
                            (*left.as_ptr()).value,
                            (*n.as_ptr()).value,
                            (*right.as_ptr()).value
                        );

                        let p = (*left.as_ptr()).parent.unwrap();
                        assert_eq!(
                            (*n.as_ptr()).value,
                            (*p.as_ptr()).value,
                            "left connection with {:?} parent is broken",
                            (*n.as_ptr()).value
                        );
                        let p = (*right.as_ptr()).parent.unwrap();
                        assert_eq!(
                            (*n.as_ptr()).value,
                            (*p.as_ptr()).value,
                            "right connection with {:?} parent is broken",
                            (*n.as_ptr()).value
                        );
                    } else if (*n.as_ptr()).left.is_some() && (*n.as_ptr()).right.is_none() {
                        if let Some(left) = (*n.as_ptr()).left {
                            println!(
                                "[{}] <- [{}] -> [NULL]",
                                (*left.as_ptr()).value,
                                (*n.as_ptr()).value
                            );
                            let p = (*left.as_ptr()).parent.unwrap();
                            assert_eq!(
                                (*n.as_ptr()).value,
                                (*p.as_ptr()).value,
                                "connection with {:?} parent is broken",
                                (*n.as_ptr()).value
                            );
                        }
                    } else if (*n.as_ptr()).left.is_none() && (*n.as_ptr()).right.is_some() {
                        if let Some(right) = (*n.as_ptr()).right {
                            println!(
                                "[NULL] <- [{}] -> [{}]",
                                (*n.as_ptr()).value,
                                (*right.as_ptr()).value
                            );
                            let p = (*right.as_ptr()).parent.unwrap();
                            assert_eq!(
                                (*n.as_ptr()).value,
                                (*p.as_ptr()).value,
                                "connection with {:?} parent is broken",
                                (*n.as_ptr()).value
                            );
                        }
                    } else {
                        println!("[{}]", (*n.as_ptr()).value);
                    }
                    checking_connections((*n.as_ptr()).left);
                    checking_connections((*n.as_ptr()).right);
                }
            }
        }
    }
}

impl<T: Ord + Default + Display> RedBlackTree<T> {
    /// Exports the tree in Graphviz DOT format.
    ///
    /// The returned string can be rendered using Graphviz-compatible tools.
    ///
    /// An empty tree produces a valid empty DOT graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbtree_extended::RedBlackTree;
    ///
    /// let mut tree = RedBlackTree::new();
    /// tree.insert(10);
    /// tree.insert(5);
    /// tree.insert(20);
    ///
    /// let dot = tree.to_dot();
    ///
    /// println!("{dot}");
    /// ```
    pub fn to_dot(&self) -> String {
        unsafe {
            let Some(fixed_head) = self.fixed_head else {
                return "digraph Tree {}".into();
            };

            let Some(root) = (*fixed_head.as_ptr()).left else {
                return "digraph Tree {}".into();
            };

            format!(
                "\ndigraph Tree {{\n\tratio = fill;\n\tnode [style=filled fontcolor=\"white\"];\n{}}}",
                helper_display_tree(root)
            )
        }
    }
}

impl<T: Ord + Default> RedBlackTree<T> {
    /// Creates an empty Red-Black Tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbtree_extended::RedBlackTree;
    ///
    /// let tree = RedBlackTree::<i32>::new();
    ///
    /// assert!(tree.is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            fixed_head: None,
            count: 0,
            _marker: PhantomData,
        }
    }

    /// Returns the number of values stored in the tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbtree_extended::RedBlackTree;
    ///
    /// let mut tree = RedBlackTree::new();
    /// tree.insert(10);
    /// tree.insert(20);
    ///
    /// assert_eq!(tree.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.count
    }

    /// Returns `true` if the tree contains no values.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbtree_extended::RedBlackTree;
    ///
    /// let mut tree = RedBlackTree::<i32>::new();
    /// assert!(tree.is_empty());
    ///
    /// tree.insert(10);
    /// assert!(!tree.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Removes all values from the tree.
    ///
    /// After calling `clear`, the tree is empty and can be reused.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbtree_extended::RedBlackTree;
    ///
    /// let mut tree = RedBlackTree::new();
    /// tree.extend([1, 2, 3]);
    ///
    /// tree.clear();
    ///
    /// assert!(tree.is_empty());
    /// assert_eq!(tree.len(), 0);
    /// ```
    pub fn clear(&mut self) {
        unsafe {
            if let Some(fixed_head) = self.fixed_head {
                let root = (*fixed_head.as_ptr()).left;

                (*fixed_head.as_ptr()).left = None;

                self.remove_tree(root);
            }

            debug_assert_eq!(self.count, 0);
        }
    }

    /// Returns the smallest value in the tree.
    ///
    /// Returns `None` if the tree is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbtree_extended::RedBlackTree;
    ///
    /// let mut tree = RedBlackTree::new();
    /// tree.extend([8, 3, 10, 1]);
    ///
    /// assert_eq!(tree.first(), Some(&1));
    /// ```
    pub fn first(&self) -> Option<&T> {
        self.minimum()
    }

    /// Returns the largest value in the tree.
    ///
    /// Returns `None` if the tree is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbtree_extended::RedBlackTree;
    ///
    /// let mut tree = RedBlackTree::new();
    /// tree.extend([8, 3, 10, 14]);
    ///
    /// assert_eq!(tree.last(), Some(&14));
    /// ```
    pub fn last(&self) -> Option<&T> {
        self.maximum()
    }

    fn minimum(&self) -> Option<&T> {
        unsafe {
            let root = self.get_root()?;
            let node = self.find_min(root);
            Some(&node.as_ref().value)
        }
    }

    fn maximum(&self) -> Option<&T> {
        unsafe {
            let root = self.get_root()?;
            let node = self.find_max(root);

            Some(&node.as_ref().value)
        }
    }

    /// Returns the height of the tree.
    ///
    /// An empty tree has height `0`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbtree_extended::RedBlackTree;
    ///
    /// let mut tree = RedBlackTree::new();
    /// assert_eq!(tree.height(), 0);
    ///
    /// tree.insert(10);
    /// assert_eq!(tree.height(), 1);
    /// ```
    pub fn height(&self) -> usize {
        unsafe fn height_from<T>(node: Link<T>) -> usize {
            unsafe {
                match node {
                    None => 0,

                    Some(node) => {
                        let left_height = height_from(node.as_ref().left);
                        let right_height = height_from(node.as_ref().right);

                        1 + left_height.max(right_height)
                    }
                }
            }
        }

        unsafe { height_from(self.get_root()) }
    }

    /// Returns `true` if the tree contains `value`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbtree_extended::RedBlackTree;
    ///
    /// let mut tree = RedBlackTree::new();
    /// tree.extend([1, 3, 5]);
    ///
    /// assert!(tree.contains(&3));
    /// assert!(!tree.contains(&4));
    /// ```
    pub fn contains(&self, value: &T) -> bool {
        !find_node(self.get_root(), value).is_none()
    }

    /// Inserts a value into the tree.
    ///
    /// Returns `true` if the value was inserted.
    ///
    /// Returns `false` if the value already exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbtree_extended::RedBlackTree;
    ///
    /// let mut tree = RedBlackTree::new();
    ///
    /// assert!(tree.insert(10));
    /// assert!(!tree.insert(10));
    /// ```
    pub fn insert(&mut self, value: T) -> bool {
        unsafe {
            if let Some(fixed_head) = self.fixed_head {
                if (*fixed_head.as_ptr()).left.is_none() {
                    (*fixed_head.as_ptr()).left = Node::new_black(value);
                    self.count += 1;
                    return true;
                }

                let parent = self.find_put_parent_candidate(self.get_root(), &value);

                if let Some(parent) = parent {
                    if self.attach_node(parent, value) {
                        self.put_balancing(parent);
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            } else {
                self.fixed_head = Node::new_fixed_head(value);
            }

            self.count += 1;
            true
        }
    }

    /// Removes a value from the tree.
    ///
    /// Returns `true` if the value existed and was removed.
    ///
    /// Use [`RedBlackTree::take`] when ownership of the removed value is needed.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbtree_extended::RedBlackTree;
    ///
    /// let mut tree = RedBlackTree::new();
    /// tree.insert(10);
    ///
    /// assert!(tree.remove(&10));
    /// assert!(!tree.contains(&10));
    /// assert!(!tree.remove(&10));
    /// ```
    pub fn remove(&mut self, value: &T) -> bool {
        self.take(value).is_some()
    }

    /// Inserts `value` into the tree, replacing an existing equal value.
    ///
    /// If the tree already contains a value that compares equal to `value`,
    /// the existing value is replaced and returned.
    ///
    /// If no equal value exists, `value` is inserted and `None` is returned.
    ///
    /// This is especially useful for custom types whose [`Ord`] implementation
    /// compares only a key field.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::cmp::Ordering;
    /// use rbtree_extended::RedBlackTree;
    ///
    /// #[derive(Debug, Default)]
    /// struct User {
    ///     id: u32,
    ///     name: String,
    /// }
    ///
    /// impl PartialEq for User {
    ///     fn eq(&self, other: &Self) -> bool {
    ///         self.id == other.id
    ///     }
    /// }
    ///
    /// impl Eq for User {}
    ///
    /// impl PartialOrd for User {
    ///     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    ///         Some(self.cmp(other))
    ///     }
    /// }
    ///
    /// impl Ord for User {
    ///     fn cmp(&self, other: &Self) -> Ordering {
    ///         self.id.cmp(&other.id)
    ///     }
    /// }
    ///
    /// let mut users = RedBlackTree::new();
    ///
    /// users.insert(User {
    ///     id: 1,
    ///     name: "Alice".into(),
    /// });
    ///
    /// let old = users.replace(User {
    ///     id: 1,
    ///     name: "Bob".into(),
    /// });
    ///
    /// assert_eq!(old.unwrap().name, "Alice");
    /// assert_eq!(users.first().unwrap().name, "Bob");
    /// ```
    pub fn replace(&mut self, value: T) -> Option<T> {
        unsafe {
            if let Some(mut node) = find_node(self.get_root(), &value) {
                return Some(std::mem::replace(&mut node.as_mut().value, value));
            }
        }

        self.insert(value);

        None
    }

    unsafe fn take_node(&mut self, mut node: NonNull<Node<T>>) -> Option<T> {
        unsafe {
            // Save the original value before the internal removal algorithm
            // starts swapping values between nodes.
            let removed_value = std::mem::take(&mut node.as_mut().value);

            if !self.remove_node(node) {
                return None;
            }

            self.count -= 1;

            Some(removed_value)
        }
    }

    /// Removes and returns the value equal to `value`.
    ///
    /// Returns `None` if the value is not present in the tree.
    ///
    /// Unlike [`RedBlackTree::remove`], this method gives ownership of the
    /// removed value back to the caller.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbtree_extended::RedBlackTree;
    ///
    /// let mut tree = RedBlackTree::new();
    /// tree.extend([10, 5, 20]);
    ///
    /// assert_eq!(tree.take(&10), Some(10));
    /// assert!(!tree.contains(&10));
    /// assert_eq!(tree.take(&100), None);
    /// ```
    pub fn take(&mut self, value: &T) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        unsafe {
            let node = find_node(self.get_root(), value)?;
            self.take_node(node)
        }
    }

    /// Retains only the values specified by the predicate.
    ///
    /// In other words, removes every value for which `f(&value)` returns `false`.
    ///
    /// The predicate receives a shared reference to each stored value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbtree_extended::RedBlackTree;
    ///
    /// let mut tree = RedBlackTree::new();
    /// tree.extend([1, 2, 3, 4, 5, 6]);
    ///
    /// tree.retain(|value| value % 2 == 0);
    ///
    /// assert_eq!(
    ///     tree.iter().copied().collect::<Vec<_>>(),
    ///     vec![2, 4, 6]
    /// );
    /// ```
    ///
    /// Retaining no values empties the tree:
    ///
    /// ```
    /// use rbtree_extended::RedBlackTree;
    ///
    /// let mut tree = RedBlackTree::new();
    /// tree.extend([1, 2, 3]);
    ///
    /// tree.retain(|_| false);
    ///
    /// assert!(tree.is_empty());
    /// ```
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> bool,
    {
        let mut retained = Self::new();

        while let Some(value) = self.pop_first() {
            if f(&value) {
                retained.insert(value);
            }
        }

        *self = retained;
    }

    /// Removes and returns the smallest value in the tree.
    ///
    /// Returns `None` if the tree is empty.
    ///
    /// This operation is useful when values need to be processed in
    /// ascending order.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbtree_extended::RedBlackTree;
    ///
    /// let mut tree = RedBlackTree::new();
    /// tree.extend([8, 3, 10, 1, 6]);
    ///
    /// assert_eq!(tree.pop_first(), Some(1));
    /// assert_eq!(tree.pop_first(), Some(3));
    /// assert_eq!(tree.len(), 3);
    /// ```
    pub fn pop_first(&mut self) -> Option<T> {
        unsafe {
            let root = self.get_root()?;
            let node = self.find_min(root);

            self.take_node(node)
        }
    }

    /// Removes and returns the largest value in the tree.
    ///
    /// Returns `None` if the tree is empty.
    ///
    /// This operation can be used to implement a priority queue where the
    /// greatest value represents the highest priority.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbtree_extended::RedBlackTree;
    ///
    /// let mut tree = RedBlackTree::new();
    /// tree.extend([8, 3, 10, 1, 6]);
    ///
    /// assert_eq!(tree.pop_last(), Some(10));
    /// assert_eq!(tree.pop_last(), Some(8));
    /// assert_eq!(tree.len(), 3);
    /// ```
    pub fn pop_last(&mut self) -> Option<T> {
        unsafe {
            let root = self.get_root()?;
            let node = self.find_max(root);

            self.take_node(node)
        }
    }

    /// Returns a reference to the stored value equal to `value`.
    ///
    /// Returns `None` if no matching value exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbtree_extended::RedBlackTree;
    ///
    /// let mut tree = RedBlackTree::new();
    /// tree.extend([1, 3, 5]);
    ///
    /// assert_eq!(tree.get(&3), Some(&3));
    /// assert_eq!(tree.get(&4), None);
    /// ```
    pub fn get(&self, value: &T) -> Option<&T> {
        unsafe {
            let node = find_node(self.get_root(), value)?;
            Some(&node.as_ref().value)
        }
    }

    /// Returns the next greater value in sorted order.
    ///
    /// Returns `None` if `value` is not present or if it is the largest value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbtree_extended::RedBlackTree;
    ///
    /// let mut tree = RedBlackTree::new();
    /// tree.extend([1, 3, 5, 7]);
    ///
    /// assert_eq!(tree.successor(&3), Some(&5));
    /// assert_eq!(tree.successor(&7), None);
    /// ```
    pub fn successor(&self, value: &T) -> Option<&T> {
        unsafe {
            let node = find_node(self.get_root(), value)?;

            // Case 1:
            // there is a right subtree - successor = minimum element on the right
            if let Some(right) = node.as_ref().right {
                let successor = self.find_min(right);
                return Some(&successor.as_ref().value);
            }

            // Case 2:
            // there is no right subtree
            // Move up while the current node is a right child
            let mut current = node;
            let mut parent = current.as_ref().parent;

            while let Some(p) = parent {
                if let Some(left) = p.as_ref().left
                    && std::ptr::eq(left.as_ptr(), current.as_ptr())
                {
                    return Some(&p.as_ref().value);
                }

                current = p;
                parent = p.as_ref().parent;
            }

            None
        }
    }

    /// Returns the previous smaller value in sorted order.
    ///
    /// Returns `None` if `value` is not present or if it is the smallest value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbtree_extended::RedBlackTree;
    ///
    /// let mut tree = RedBlackTree::new();
    /// tree.extend([1, 3, 5, 7]);
    ///
    /// assert_eq!(tree.predecessor(&5), Some(&3));
    /// assert_eq!(tree.predecessor(&1), None);
    /// ```
    pub fn predecessor(&self, value: &T) -> Option<&T> {
        unsafe {
            let node = find_node(self.get_root(), value)?;

            // Case 1:
            // there is a left subtree - predecessor = maximum element on the left
            if let Some(left) = node.as_ref().left {
                let predecessor = self.find_max(left);
                return Some(&predecessor.as_ref().value);
            }

            // Case 2:
            // there is no left subtree
            // Move up while the current node is a left child
            let mut current = node;
            let mut parent = current.as_ref().parent;

            while let Some(p) = parent {
                if let Some(right) = p.as_ref().right
                    && std::ptr::eq(right.as_ptr(), current.as_ptr())
                {
                    return Some(&p.as_ref().value);
                }

                current = p;
                parent = p.as_ref().parent;
            }

            None
        }
    }

    /// Returns the first value greater than or equal to `value`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbtree_extended::RedBlackTree;
    ///
    /// let mut tree = RedBlackTree::new();
    /// tree.extend([1, 3, 6, 8]);
    ///
    /// assert_eq!(tree.lower_bound(&5), Some(&6));
    /// assert_eq!(tree.lower_bound(&6), Some(&6));
    /// ```
    pub fn lower_bound(&self, value: &T) -> Option<&T> {
        unsafe {
            let mut current = self.get_root();
            let mut candidate: Link<T> = None;

            while let Some(node) = current {
                match node.as_ref().value.cmp(value) {
                    Ordering::Less => {
                        current = node.as_ref().right;
                    }

                    Ordering::Equal => {
                        return Some(&node.as_ref().value);
                    }

                    Ordering::Greater => {
                        candidate = Some(node);
                        current = node.as_ref().left;
                    }
                }
            }

            candidate.map(|node| &node.as_ref().value)
        }
    }

    /// Returns the first value strictly greater than `value`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbtree_extended::RedBlackTree;
    ///
    /// let mut tree = RedBlackTree::new();
    /// tree.extend([1, 3, 6, 8]);
    ///
    /// assert_eq!(tree.upper_bound(&6), Some(&8));
    /// assert_eq!(tree.upper_bound(&8), None);
    /// ```
    pub fn upper_bound(&self, value: &T) -> Option<&T> {
        unsafe {
            let mut current = self.get_root();
            let mut candidate: Link<T> = None;

            while let Some(node) = current {
                match node.as_ref().value.cmp(value) {
                    Ordering::Less | Ordering::Equal => {
                        current = node.as_ref().right;
                    }
                    Ordering::Greater => {
                        candidate = Some(node);
                        current = node.as_ref().left;
                    }
                }
            }

            candidate.map(|node| &node.as_ref().value)
        }
    }

    /// Returns an iterator over values in the inclusive range `[min, max]`.
    ///
    /// Values are yielded in ascending order.
    ///
    /// The iterator is lazy: values are produced only as they are requested,
    /// rather than being collected into an intermediate `Vec`.
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
    #[cfg(feature = "in-order")]
    pub fn range(&self, min: &T, max: &T) -> Range<'_, T> {
        Range::new(self.get_root(), min, max)
    }

    /// Returns the next value in an in-order (`LNR`) traversal.
    ///
    /// For a binary search tree, in-order traversal visits values in ascending order.
    ///
    /// Returns `None` if `value` is not present or has no successor in this traversal.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbtree_extended::RedBlackTree;
    ///
    /// let mut tree = RedBlackTree::new();
    /// tree.extend([4, 2, 6, 1, 3, 5, 7]);
    ///
    /// assert_eq!(tree.successor_dfs_in_order(4), Some(&5));
    /// ```
    #[cfg(feature = "in-order")]
    pub fn successor_dfs_in_order(&self, value: T) -> Option<&T> {
        unsafe {
            let node = find_node(self.get_root(), &value);
            if let Some(n) = node
                && let Some(nodesucc) = successor_of_node_in_order(n)
            {
                return Some(&nodesucc.as_ref().value);
            }
            None
        }
    }

    /// Returns the next value in reverse in-order (`RNL`) traversal.
    ///
    /// For a binary search tree, reverse in-order traversal visits values in
    /// descending order.
    ///
    /// Returns `None` if `value` is not present or has no successor in this traversal.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbtree_extended::RedBlackTree;
    ///
    /// let mut tree = RedBlackTree::new();
    /// tree.extend([4, 2, 6, 1, 3, 5, 7]);
    ///
    /// assert_eq!(tree.successor_dfs_reverse_in_order(6), Some(&5));
    /// ```
    #[cfg(feature = "reverse-in-order")]
    pub fn successor_dfs_reverse_in_order(&self, value: T) -> Option<&T> {
        unsafe {
            if let Some(node) = find_node(self.get_root(), &value)
                && let Some(next) = successor_of_node_reverse_in_order(node)
            {
                return Some(&next.as_ref().value);
            }
            None
        }
    }

    /// Returns the next value in post-order (`LRN`) traversal.
    ///
    /// Post-order visits the left subtree, then the right subtree,
    /// and finally the current node.
    ///
    /// Returns `None` if `value` is not present or has no successor in this traversal.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbtree_extended::RedBlackTree;
    ///
    /// let mut tree = RedBlackTree::new();
    /// tree.extend([4, 2, 6, 1, 3, 5, 7]);
    ///
    /// // Post-order: 1, 3, 2, 5, 7, 6, 4
    /// assert_eq!(tree.successor_dfs_post_order(3), Some(&2));
    /// assert_eq!(tree.successor_dfs_post_order(2), Some(&5));
    /// assert_eq!(tree.successor_dfs_post_order(4), None);
    /// ```
    #[cfg(feature = "post-order")]
    pub fn successor_dfs_post_order(&self, value: T) -> Option<&T> {
        unsafe {
            if let Some(n) = find_node(self.get_root(), &value)
                && let Some(nodesucc) = successor_of_node_post_order(n)
            {
                return Some(&nodesucc.as_ref().value);
            }
            None
        }
    }

    /// Returns the next value in pre-order (`NLR`) traversal.
    ///
    /// Pre-order visits the current node first, followed by the left and right
    /// subtrees.
    ///
    /// Returns `None` if `value` is not present or has no successor in this traversal.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbtree_extended::RedBlackTree;
    ///
    /// let mut tree = RedBlackTree::new();
    /// tree.extend([4, 2, 6, 1, 3, 5, 7]);
    ///
    /// let first = *tree.iter_preorder().next().unwrap();
    /// assert_eq!(tree.successor_dfs_pre_order(first), tree.iter_preorder().nth(1));
    /// ```
    #[cfg(feature = "pre-order")]
    pub fn successor_dfs_pre_order(&self, value: T) -> Option<&T> {
        unsafe {
            if let Some(n) = find_node(self.get_root(), &value)
                && let Some(nodesucc) = successor_of_node_pre_order(n)
            {
                return Some(&nodesucc.as_ref().value);
            }
            None
        }
    }

    fn get_root(&self) -> Link<T> {
        if let Some(fixed_head) = self.fixed_head {
            unsafe { (*fixed_head.as_ptr()).left }
        } else {
            None
        }
    }

    unsafe fn operation_remove(&self, node_x: &NonNull<Node<T>>) -> OperationRemove {
        unsafe {
            if node_x.as_ref().is_red
                && node_x.as_ref().left.is_none()
                && node_x.as_ref().right.is_none()
            {
                return OperationRemove::RedLeaf;
            }
            if node_x.as_ref().is_red
                && node_x.as_ref().left.is_some()
                && node_x.as_ref().right.is_some()
            {
                return OperationRemove::NodeWithChildren;
            }
            if !node_x.as_ref().is_red {
                if node_x.as_ref().left.is_some() && node_x.as_ref().right.is_some() {
                    return OperationRemove::NodeWithChildren;
                } else if node_x.as_ref().left.is_some() && node_x.as_ref().right.is_none() {
                    if let Some(left) = node_x.as_ref().left
                        && left.as_ref().is_red
                        && left.as_ref().left.is_none()
                        && left.as_ref().right.is_none()
                    {
                        return OperationRemove::BlackNodeWithRedLeaf;
                    }
                } else if node_x.as_ref().left.is_none() && node_x.as_ref().right.is_none() {
                    return OperationRemove::BlackLeaf;
                }
            }
            panic!();
        }
    }

    unsafe fn operation_remove_black_leaf(
        &self,
        node_x: &NonNull<Node<T>>,
    ) -> OperationRemoveBlackLeaf {
        unsafe {
            if let Some(node_a) = node_x.as_ref().parent {
                if node_a.as_ref().is_red {
                    if let Some(node_b) = node_a.as_ref().left
                        && !std::ptr::eq(node_b.as_ptr(), node_x.as_ptr())
                        && !node_b.as_ref().is_red
                    {
                        if let Some(node_c) = node_b.as_ref().left
                            && node_c.as_ref().is_red
                        {
                            return OperationRemoveBlackLeaf::LeftRedABlackBRedCleaf;
                            // 2.1.1.1
                        }

                        // sibling B is black and its left child is either absent or also black
                        return OperationRemoveBlackLeaf::LeftRedABlackBleaf;
                        // 2.1.2.1
                    }

                    if let Some(node_b) = node_a.as_ref().right
                        && !std::ptr::eq(node_b.as_ptr(), node_x.as_ptr())
                        && !node_b.as_ref().is_red
                    {
                        if let Some(node_c) = node_b.as_ref().left
                            && node_c.as_ref().is_red
                        {
                            return OperationRemoveBlackLeaf::RightRedABlackBRedCleaf;
                            // 2.1.1.2
                        }

                        // sibling B is black and its left child is either absent or also black
                        return OperationRemoveBlackLeaf::RightRedABlackBleaf;
                        // 2.1.2.2
                    }
                } else {
                    if let Some(node_b) = node_a.as_ref().left
                        && !std::ptr::eq(node_b.as_ptr(), node_x.as_ptr())
                    {
                        if node_b.as_ref().is_red && node_b.as_ref().right.is_some() {
                            let node_c = node_b.as_ref().right.unwrap();
                            if !node_c.as_ref().is_red {
                                if node_c.as_ref().left.is_some()
                                    && let Some(node_d) = node_c.as_ref().left
                                    && node_d.as_ref().is_red
                                {
                                    return OperationRemoveBlackLeaf::BlackARedBWithBlackChildrenRightHaveRedLeaf;
                                    // 2.2.1
                                }
                                return OperationRemoveBlackLeaf::BlackARedBWithBlackChildrenLeaf;
                                // 2.2.2
                            }
                        }
                        if !node_b.as_ref().is_red
                            && node_b.as_ref().left.is_some()
                            && let Some(node_d) = node_b.as_ref().left
                            && node_d.as_ref().is_red
                        {
                            return OperationRemoveBlackLeaf::BlackALeftBlackBRedDleaf;
                            // 2.3.1.1
                        }
                        if !node_b.as_ref().is_red {
                            return OperationRemoveBlackLeaf::BlackALeftBlackBleaf;
                            // 2.3.2.1
                        }
                    }
                    if let Some(node_b) = node_a.as_ref().right
                        && !std::ptr::eq(node_b.as_ptr(), node_x.as_ptr())
                    {
                        if !node_b.as_ref().is_red
                            && node_b.as_ref().left.is_some()
                            && let Some(node_d) = node_b.as_ref().left
                            && node_d.as_ref().is_red
                        {
                            return OperationRemoveBlackLeaf::BlackARightBlackBRedDleaf;
                            // 2.3.1.2
                        }
                        if !node_b.as_ref().is_red {
                            return OperationRemoveBlackLeaf::BlackARightBlackBleaf;
                            // 2.3.2.2
                        }
                    }
                }
            } else {
                return OperationRemoveBlackLeaf::Root;
            }
            OperationRemoveBlackLeaf::Unimplemented
        }
    }

    unsafe fn remove_black_leaf(&mut self, node_x: NonNull<Node<T>>) -> bool {
        unsafe {
            let mut next = node_x;
            let mut removed = false;
            loop {
                match self.operation_remove_black_leaf(&next) {
                    OperationRemoveBlackLeaf::LeftRedABlackBRedCleaf => {
                        self.remove_black_leaf_2_1_1_1_balancing(&next);
                        if !removed {
                            removed = self.remove_leaf(node_x);
                        }
                        break;
                    }
                    OperationRemoveBlackLeaf::RightRedABlackBRedCleaf => {
                        self.remove_black_leaf_2_1_1_2_balancing(&next);
                        if !removed {
                            removed = self.remove_leaf(node_x);
                        }
                        break;
                    }
                    OperationRemoveBlackLeaf::LeftRedABlackBleaf => {
                        self.remove_black_leaf_2_1_2_1_balancing(&next);
                        if !removed {
                            removed = self.remove_leaf(node_x);
                        }
                        break;
                    }
                    OperationRemoveBlackLeaf::RightRedABlackBleaf => {
                        self.remove_black_leaf_2_1_2_2_balancing(&next);
                        if !removed {
                            removed = self.remove_leaf(node_x);
                        }
                        break;
                    }
                    OperationRemoveBlackLeaf::BlackARedBWithBlackChildrenRightHaveRedLeaf => {
                        self.remove_black_leaf_2_2_1_balancing(&next);
                        if !removed {
                            removed = self.remove_leaf(node_x);
                        }
                        break;
                    }
                    OperationRemoveBlackLeaf::BlackARedBWithBlackChildrenLeaf => {
                        self.remove_black_leaf_2_2_2_balancing(&next);
                        if !removed {
                            removed = self.remove_leaf(node_x);
                        }
                        break;
                    }
                    OperationRemoveBlackLeaf::BlackALeftBlackBRedDleaf => {
                        self.remove_black_leaf_2_3_1_1_balancing(&next);
                        if !removed {
                            removed = self.remove_leaf(node_x);
                        }
                        break;
                    }
                    OperationRemoveBlackLeaf::BlackARightBlackBRedDleaf => {
                        self.remove_black_leaf_2_3_1_2_balancing(&next);
                        if !removed {
                            removed = self.remove_leaf(node_x);
                        }
                        break;
                    }
                    OperationRemoveBlackLeaf::BlackALeftBlackBleaf => {
                        next = self.remove_black_leaf_2_3_2_1_balancing(&next);
                        if !removed {
                            removed = self.remove_leaf(node_x);
                        }
                    }
                    OperationRemoveBlackLeaf::BlackARightBlackBleaf => {
                        next = self.remove_black_leaf_2_3_2_2_balancing(&next);
                        if !removed {
                            removed = self.remove_leaf(node_x);
                        }
                    }
                    OperationRemoveBlackLeaf::Root => {
                        if !removed {
                            // We get here when the node_x being deleted is root
                            if let Some(fixed_head) = self.fixed_head {
                                (*fixed_head.as_ptr()).left = None;
                            }
                            let _ = Box::from_raw(node_x.as_ptr());
                            removed = true;
                        }
                        break;
                    }
                    OperationRemoveBlackLeaf::Unimplemented => {
                        panic!("unimplemented black leaf removal case");
                    }
                }
            }
            removed
        }
    }

    /// 2.1.1.1 remove black X
    ///
    /// ```text
    ///             P            P
    ///            //           //
    ///            A            B
    ///           / \          / \
    ///          B   X   =>   C   A
    ///         //\              /
    ///        C   D            D
    /// ```
    unsafe fn remove_black_leaf_2_1_1_1_balancing(&mut self, node_x: &NonNull<Node<T>>) {
        unsafe {
            let node_a = (*node_x.as_ptr()).parent.unwrap();
            let node_b = (*node_a.as_ptr()).left.unwrap();
            let node_d = (*node_b.as_ptr()).right;
            (*node_b.as_ptr()).parent = (*node_a.as_ptr()).parent;
            if let Some(ref mut parent) = (*node_b.as_ptr()).parent {
                (*parent.as_ptr()).left = Some(node_b);
            }
            (*node_a.as_ptr()).left = node_d;
            if let Some(n_d) = node_d {
                (*n_d.as_ptr()).parent = Some(node_a);
            }
            (*node_a.as_ptr()).is_red = false;
            (*node_b.as_ptr()).right = Some(node_a);
            (*node_a.as_ptr()).parent = Some(node_b);
            (*node_b.as_ptr()).is_red = true;
            if let Some(node_c) = (*node_b.as_ptr()).left {
                (*node_c.as_ptr()).is_red = false;
            }
        }
    }

    /// 2.1.1.2 remove black X
    ///
    /// ```text
    ///             P              P
    ///            //             //
    ///           A               C
    ///          / \            /   \
    ///         X   B   =>     A     B
    ///            //           \   /
    ///            C             E D
    ///           / \
    ///          E   D
    /// ```
    unsafe fn remove_black_leaf_2_1_1_2_balancing(&mut self, node_x: &NonNull<Node<T>>) {
        unsafe {
            let node_a = (*node_x.as_ptr()).parent.unwrap();
            (*node_a.as_ptr()).is_red = false;
            let parent: NonNull<Node<T>> = (*node_a.as_ptr()).parent.unwrap();
            let node_b = (*node_a.as_ptr()).right.unwrap();
            let node_c = (*node_b.as_ptr()).left.unwrap();
            let node_e = (*node_c.as_ptr()).left;
            let node_d = (*node_c.as_ptr()).right;
            (*parent.as_ptr()).left = Some(node_c);
            (*node_c.as_ptr()).parent = Some(parent);
            (*node_b.as_ptr()).left = node_d;
            if let Some(n_d) = node_d {
                (*n_d.as_ptr()).parent = Some(node_b);
            }
            (*node_c.as_ptr()).right = Some(node_b);
            (*node_b.as_ptr()).parent = Some(node_c);
            (*node_c.as_ptr()).left = Some(node_a);
            (*node_a.as_ptr()).parent = Some(node_c);
            (*node_a.as_ptr()).right = node_e;
            if let Some(n_e) = node_e {
                (*n_e.as_ptr()).parent = Some(node_a);
            }
        }
    }

    /// 2.1.2.1 remove black X
    ///
    /// ```text
    ///              P            P
    ///             //           /
    ///            A            A
    ///           / \          //
    ///          B   X   =>   B
    ///
    /// ```
    unsafe fn remove_black_leaf_2_1_2_1_balancing(&mut self, node_x: &NonNull<Node<T>>) {
        unsafe {
            let node_a = (*node_x.as_ptr()).parent.unwrap();
            (*node_a.as_ptr()).is_red = false;
            if let Some(node_b) = (*node_a.as_ptr()).left {
                (*node_b.as_ptr()).is_red = true;
            }
        }
    }

    /// 2.1.2.2 remove black X
    ///
    /// ```text
    ///             P            P
    ///            //           /
    ///            A     =>     B
    ///           / \          //\
    ///          X   B        A   D
    ///             / \        \
    ///            C   D        C
    /// ```
    unsafe fn remove_black_leaf_2_1_2_2_balancing(&mut self, node_x: &NonNull<Node<T>>) {
        unsafe {
            let node_a = (*node_x.as_ptr()).parent.unwrap();
            let node_b = (*node_a.as_ptr()).right.unwrap();
            let node_c = (*node_b.as_ptr()).left;
            let node_d = (*node_b.as_ptr()).right;
            let parent = (*node_a.as_ptr()).parent.unwrap();
            if let Some(left) = (*parent.as_ptr()).left {
                if std::ptr::eq(node_a.as_ptr(), left.as_ptr()) {
                    (*parent.as_ptr()).left = Some(node_b);
                } else {
                    (*parent.as_ptr()).right = Some(node_b);
                }
            } else {
                (*parent.as_ptr()).right = Some(node_b);
            }
            (*node_b.as_ptr()).parent = Some(parent);
            (*node_b.as_ptr()).left = Some(node_a);
            (*node_b.as_ptr()).right = node_d;
            if let Some(n_d) = node_d {
                (*n_d.as_ptr()).parent = Some(node_b);
            }
            (*node_a.as_ptr()).parent = Some(node_b);
            (*node_a.as_ptr()).right = node_c;
            if let Some(n_c) = node_c {
                (*n_c.as_ptr()).parent = Some(node_a);
            }
        }
    }

    /// 2.2.1 remove black X
    ///
    /// ```text
    ///            A               C
    ///          // \            //  \
    ///         B    X    =>    B     A
    ///          \               \   /
    ///           C               D E
    ///           //\
    ///          D   E
    /// ```
    unsafe fn remove_black_leaf_2_2_1_balancing(&mut self, node_x: &NonNull<Node<T>>) {
        unsafe {
            let n_a = node_x.as_ref().parent.unwrap();
            if n_a.as_ref().parent.is_some() {
                let parent = (*n_a.as_ptr()).parent.unwrap();
                let mut node_a_from_left = false;
                if let Some(ref mut node_a) = (*parent.as_ptr()).left
                    && std::ptr::eq(node_a.as_ptr(), n_a.as_ptr())
                {
                    node_a_from_left = true;
                }
                let node_a = if node_a_from_left {
                    (*parent.as_ptr()).left.unwrap()
                } else {
                    (*parent.as_ptr()).right.unwrap()
                };
                let node_b = (*node_a.as_ptr()).left.unwrap();
                let node_c = (*node_b.as_ptr()).right.unwrap();
                let node_e = (*node_c.as_ptr()).right;
                let node_d = (*node_c.as_ptr()).left.unwrap();
                (*node_b.as_ptr()).right = Some(node_d);
                (*node_b.as_ptr()).parent = Some(node_c);
                (*node_d.as_ptr()).parent = Some(node_b);
                (*node_a.as_ptr()).parent = Some(node_c);
                (*node_a.as_ptr()).left = node_e;
                if let Some(n_e) = node_e {
                    (*n_e.as_ptr()).parent = Some(node_a);
                }
                (*node_d.as_ptr()).is_red = false;
                (*node_c.as_ptr()).left = Some(node_b);
                (*node_c.as_ptr()).right = Some(node_a);
                (*node_c.as_ptr()).parent = Some(parent);
                if node_a_from_left {
                    (*parent.as_ptr()).left = Some(node_c);
                } else {
                    (*parent.as_ptr()).right = Some(node_c);
                }
            } else {
                // node_a is root
                if let Some(fixed_head) = self.fixed_head {
                    let node_a = (*fixed_head.as_ptr()).left.unwrap();
                    let node_b = (*node_a.as_ptr()).left.unwrap();
                    let node_c = (*node_b.as_ptr()).right.unwrap();
                    let node_e = (*node_c.as_ptr()).right;
                    let node_d = (*node_c.as_ptr()).left.unwrap();
                    (*node_b.as_ptr()).right = Some(node_d);
                    (*node_b.as_ptr()).parent = Some(node_c);
                    (*node_d.as_ptr()).parent = Some(node_b);
                    (*node_a.as_ptr()).parent = Some(node_c);
                    (*node_a.as_ptr()).left = node_e;
                    if let Some(n_e) = node_e {
                        (*n_e.as_ptr()).parent = Some(node_a);
                    }
                    (*node_d.as_ptr()).is_red = false;
                    (*node_c.as_ptr()).left = Some(node_b);
                    (*node_c.as_ptr()).right = Some(node_a);
                    (*node_c.as_ptr()).parent = None;
                    (*fixed_head.as_ptr()).left = Some(node_c);
                }
            }
        }
    }

    /// 2.2.2 remove black X
    ///
    /// ```text
    ///    A            B
    ///  // \            \
    ///  B    X   =>       A
    ///  \               //
    ///   C             C
    /// ```
    unsafe fn remove_black_leaf_2_2_2_balancing(&mut self, node_x: &NonNull<Node<T>>) {
        unsafe {
            let n_a = node_x.as_ref().parent.unwrap();
            if n_a.as_ref().parent.is_some() {
                let parent = (*n_a.as_ptr()).parent.unwrap();
                let mut node_a_from_left = false;
                if let Some(ref mut node_a) = (*parent.as_ptr()).left
                    && std::ptr::eq(node_a.as_ptr(), n_a.as_ptr())
                {
                    node_a_from_left = true;
                }
                let node_a = if node_a_from_left {
                    (*parent.as_ptr()).left.unwrap()
                } else {
                    (*parent.as_ptr()).right.unwrap()
                };
                let node_b = (*node_a.as_ptr()).left.unwrap();
                (*node_b.as_ptr()).is_red = false;
                (*node_b.as_ptr()).parent = Some(parent);
                let node_c = (*node_b.as_ptr()).right.unwrap();
                (*node_c.as_ptr()).is_red = true;
                (*node_b.as_ptr()).right = Some(node_a);
                (*node_a.as_ptr()).left = Some(node_c);
                (*node_c.as_ptr()).parent = Some(node_a);
                (*node_a.as_ptr()).parent = Some(node_b);
                if node_a_from_left {
                    (*parent.as_ptr()).left = Some(node_b);
                } else {
                    (*parent.as_ptr()).right = Some(node_b);
                }
            } else {
                // node_a is root
                if let Some(fixed_head) = self.fixed_head {
                    let node_a = (*fixed_head.as_ptr()).left.unwrap();
                    let node_b = (*node_a.as_ptr()).left.unwrap();
                    (*node_b.as_ptr()).is_red = false;
                    (*node_b.as_ptr()).parent = None;
                    let node_c = (*node_b.as_ptr()).right.unwrap();
                    (*node_c.as_ptr()).is_red = true;
                    (*node_b.as_ptr()).right = Some(node_a);
                    (*node_a.as_ptr()).left = Some(node_c);
                    (*node_c.as_ptr()).parent = Some(node_a);
                    (*node_a.as_ptr()).parent = Some(node_b);
                    (*fixed_head.as_ptr()).left = Some(node_b);
                }
            }
        }
    }

    /// 2.3.1.1 remove black X
    ///
    /// ```text
    ///      A           B
    ///     / \         / \
    ///    B   X  =>   D   A
    ///   // \            /
    ///  D    C          C
    /// ```
    unsafe fn remove_black_leaf_2_3_1_1_balancing(&mut self, node_x: &NonNull<Node<T>>) {
        unsafe {
            let node_a = (*node_x.as_ptr()).parent.unwrap();
            if let Some(parent) = (*node_a.as_ptr()).parent {
                let node_b = (*node_a.as_ptr()).left.unwrap();
                let node_c = (*node_b.as_ptr()).right;
                let node_d = (*node_b.as_ptr()).left.unwrap();
                (*node_d.as_ptr()).is_red = false;
                let mut node_a_from_left = false;
                if let Some(p_node_a) = (*parent.as_ptr()).left
                    && std::ptr::eq(p_node_a.as_ptr(), node_a.as_ptr())
                {
                    node_a_from_left = true;
                }
                (*node_b.as_ptr()).parent = Some(parent);
                (*node_b.as_ptr()).right = Some(node_a);
                (*node_a.as_ptr()).parent = Some(node_b);
                (*node_a.as_ptr()).left = node_c;
                if let Some(n_c) = node_c {
                    (*n_c.as_ptr()).parent = Some(node_a);
                }
                if node_a_from_left {
                    (*parent.as_ptr()).left = Some(node_b);
                } else {
                    (*parent.as_ptr()).right = Some(node_b);
                }
            } else {
                // node_a is root
                if let Some(fixed_head) = self.fixed_head {
                    let node_a = (*fixed_head.as_ptr()).left.unwrap();
                    let node_b = (*node_a.as_ptr()).left.unwrap();
                    let node_c = (*node_b.as_ptr()).right;
                    let node_d = (*node_b.as_ptr()).left.unwrap();
                    (*node_d.as_ptr()).is_red = false;
                    let node_b = (*node_a.as_ptr()).left.unwrap();
                    (*node_b.as_ptr()).parent = None;
                    (*node_b.as_ptr()).right = Some(node_a);
                    (*node_a.as_ptr()).parent = Some(node_b);
                    (*node_a.as_ptr()).left = node_c;
                    if let Some(n_c) = node_c {
                        (*n_c.as_ptr()).parent = Some(node_a);
                    }
                    (*fixed_head.as_ptr()).left = Some(node_b);
                }
            }
        }
    }

    /// 2.3.1.2 remove black X
    ///
    /// ```text
    ///
    ///      A            D
    ///     / \         /   \
    ///    X   B  =>   A     B
    ///       //        \   /
    ///       D          C E
    ///      / \
    ///     C   E
    /// ```
    unsafe fn remove_black_leaf_2_3_1_2_balancing(&mut self, node_x: &NonNull<Node<T>>) {
        unsafe {
            let node_a = (*node_x.as_ptr()).parent.unwrap();
            if let Some(parent) = (*node_a.as_ptr()).parent {
                let node_b = (*node_a.as_ptr()).right.unwrap();
                let node_d = (*node_b.as_ptr()).left.unwrap();
                let node_c = (*node_d.as_ptr()).left;
                let node_e = (*node_d.as_ptr()).right;
                (*node_d.as_ptr()).is_red = false;
                let mut node_a_from_left = false;
                if let Some(p_node_a) = (*parent.as_ptr()).left
                    && std::ptr::eq(p_node_a.as_ptr(), node_a.as_ptr())
                {
                    node_a_from_left = true;
                }
                (*node_d.as_ptr()).parent = Some(parent);
                (*node_d.as_ptr()).left = Some(node_a);
                (*node_d.as_ptr()).right = Some(node_b);
                (*node_a.as_ptr()).parent = Some(node_d);
                (*node_b.as_ptr()).parent = Some(node_d);
                (*node_a.as_ptr()).right = node_c;
                if let Some(n_c) = node_c {
                    (*n_c.as_ptr()).parent = Some(node_a);
                }
                (*node_b.as_ptr()).left = node_e;
                if let Some(n_e) = node_e {
                    (*n_e.as_ptr()).parent = Some(node_b);
                }
                if node_a_from_left {
                    (*parent.as_ptr()).left = Some(node_d);
                } else {
                    (*parent.as_ptr()).right = Some(node_d);
                }
            } else {
                // node_a is root
                if let Some(fixed_head) = self.fixed_head {
                    let node_a = (*fixed_head.as_ptr()).left.unwrap();
                    let node_b = (*node_a.as_ptr()).right.unwrap();
                    let node_d = (*node_b.as_ptr()).left.unwrap();
                    let node_c = (*node_d.as_ptr()).left;
                    let node_e = (*node_d.as_ptr()).right;
                    (*node_d.as_ptr()).is_red = false;
                    (*node_d.as_ptr()).parent = None;
                    (*node_d.as_ptr()).left = Some(node_a);
                    (*node_d.as_ptr()).right = Some(node_b);
                    (*node_a.as_ptr()).parent = Some(node_d);
                    (*node_b.as_ptr()).parent = Some(node_d);
                    (*node_a.as_ptr()).right = node_c;
                    if let Some(n_c) = node_c {
                        (*n_c.as_ptr()).parent = Some(node_a);
                    }
                    (*node_b.as_ptr()).left = node_e;
                    if let Some(n_e) = node_e {
                        (*n_e.as_ptr()).parent = Some(node_b);
                    }
                    (*fixed_head.as_ptr()).left = Some(node_d);
                }
            }
        }
    }

    /// 2.3.2.1 remove black X
    ///
    /// ```text
    ///            P           P
    ///            |           |
    ///            A           A
    ///           / \   =>    //   => next check node A
    ///          B   X       B
    ///
    /// ```
    unsafe fn remove_black_leaf_2_3_2_1_balancing(
        &mut self,
        node_x: &NonNull<Node<T>>,
    ) -> NonNull<Node<T>> {
        unsafe {
            let node_a = (*node_x.as_ptr()).parent.unwrap();
            let node_b = (*node_a.as_ptr()).left.unwrap();
            (*node_b.as_ptr()).is_red = true;
            node_a
        }
    }

    /// 2.3.2.2 remove black X
    ///
    /// ```text
    ///              A            B
    ///             / \   =>    // \    =>  next check node B
    ///            X   B       A    D
    ///               / \       \
    ///              C   D       C
    /// ```
    unsafe fn remove_black_leaf_2_3_2_2_balancing(
        &mut self,
        node_x: &NonNull<Node<T>>,
    ) -> NonNull<Node<T>> {
        unsafe {
            let node_a = (*node_x.as_ptr()).parent.unwrap();
            (*node_a.as_ptr()).is_red = true;
            if let Some(parent) = (*node_a.as_ptr()).parent {
                let node_b = (*node_a.as_ptr()).right.unwrap();
                let node_c = (*node_b.as_ptr()).left;
                let node_d = (*node_b.as_ptr()).right;
                let mut node_a_from_left = false;
                if let Some(p_node_a) = (*parent.as_ptr()).left
                    && std::ptr::eq(p_node_a.as_ptr(), node_a.as_ptr())
                {
                    node_a_from_left = true;
                }
                (*node_b.as_ptr()).parent = Some(parent);
                (*node_a.as_ptr()).right = node_c;
                if let Some(n_c) = node_c {
                    (*n_c.as_ptr()).parent = Some(node_a);
                }
                (*node_b.as_ptr()).right = node_d;
                if let Some(n_d) = node_d {
                    (*n_d.as_ptr()).parent = Some(node_b);
                }
                (*node_b.as_ptr()).left = Some(node_a);
                (*node_a.as_ptr()).parent = Some(node_b);
                if node_a_from_left {
                    (*parent.as_ptr()).left = Some(node_b);
                } else {
                    (*parent.as_ptr()).right = Some(node_b);
                }
                node_b
            } else {
                // node_a is root
                if let Some(fixed_head) = self.fixed_head {
                    let node_a = (*fixed_head.as_ptr()).left.unwrap();
                    (*node_a.as_ptr()).is_red = true;
                    let node_b = (*node_a.as_ptr()).right.unwrap();
                    let node_c = (*node_b.as_ptr()).left;
                    let node_d = (*node_b.as_ptr()).right;
                    (*node_b.as_ptr()).parent = None;
                    (*node_a.as_ptr()).right = node_c;
                    if let Some(n_c) = node_c {
                        (*n_c.as_ptr()).parent = Some(node_a);
                    }
                    (*node_b.as_ptr()).right = node_d;
                    if let Some(n_d) = node_d {
                        (*n_d.as_ptr()).parent = Some(node_b);
                    }
                    (*node_b.as_ptr()).left = Some(node_a);
                    (*node_a.as_ptr()).parent = Some(node_b);
                    (*fixed_head.as_ptr()).left = Some(node_b);
                }
                self.get_root().unwrap()
            }
        }
    }

    unsafe fn remove_node(&mut self, node: NonNull<Node<T>>) -> bool {
        unsafe {
            match self.operation_remove(&node) {
                OperationRemove::RedLeaf => {
                    // option 1.0.0
                    //
                    // ```text
                    //
                    //  P
                    //  ||
                    //  X
                    //
                    // ```

                    self.remove_leaf(node)
                }
                OperationRemove::BlackLeaf => {
                    // option 2 black leaf
                    self.remove_black_leaf(node)
                }
                OperationRemove::NodeWithChildren => {
                    // option 4.0.0
                    // ```text
                    //        X
                    //     /      \
                    //    L        R
                    //   / \      / \
                    // ...  C    L
                    //          /
                    //         C
                    //        ...
                    //
                    // ```
                    /*
                        The node has two children.

                        We take the predecessor:
                        the maximal node of the left subtree.

                        We only change the values,
                        then we delete the predecessor as a regular node.
                    */

                    let predecessor = self.find_max(node.as_ref().left.unwrap());

                    std::mem::swap(
                        &mut (*node.as_ptr()).value,
                        &mut (*predecessor.as_ptr()).value,
                    );

                    self.remove_node(predecessor)
                }
                OperationRemove::BlackNodeWithRedLeaf => {
                    // option 3.0.0
                    // ```text
                    //  X
                    // //
                    // A
                    // ```
                    let red_left = (*node.as_ptr()).left.unwrap();
                    std::mem::swap(&mut (*node.as_ptr()).value, &mut (*red_left.as_ptr()).value);
                    self.remove_leaf(red_left)
                }
            }
        }
    }

    unsafe fn find_max(&self, node: NonNull<Node<T>>) -> NonNull<Node<T>> {
        unsafe {
            if node.as_ref().right.is_some() {
                self.find_max(node.as_ref().right.unwrap())
            } else {
                node
            }
        }
    }

    unsafe fn find_min(&self, node: NonNull<Node<T>>) -> NonNull<Node<T>> {
        unsafe {
            if node.as_ref().left.is_some() {
                self.find_min(node.as_ref().left.unwrap())
            } else {
                node
            }
        }
    }

    unsafe fn remove_leaf(&mut self, node: NonNull<Node<T>>) -> bool {
        unsafe {
            if let Some(mut parent) = node.as_ref().parent {
                if let Some(ref mut left) = parent.as_mut().left {
                    if std::ptr::eq(left.as_ptr(), node.as_ptr()) {
                        parent.as_mut().left = None;
                    } else {
                        parent.as_mut().right = None;
                    }
                } else {
                    parent.as_mut().right = None;
                }
            }
            let _ = Box::from_raw(node.as_ptr());
            true
        }
    }

    unsafe fn remove_tree(&mut self, node: Link<T>) {
        unsafe {
            if let Some(node) = node {
                self.remove_tree(node.as_ref().left);
                self.remove_tree(node.as_ref().right);
                if self.remove_leaf(node) {
                    assert!(self.count > 0);
                    self.count -= 1;
                }
            }
        }
    }

    /// Rotate left and Rotate right and Flip color
    ///
    /// ```text
    ///             |              ||
    ///             P              C
    ///           //  \          /   \
    ///          A     F        A     P
    ///         / \\      =>   / \   / \
    ///        B   C          B   E D   F
    ///            / \
    ///           E   D
    /// ```
    unsafe fn rotate_left_right_flip_color(&mut self, node_a: NonNull<Node<T>>) -> Link<T> {
        unsafe {
            let node_p = (*node_a.as_ptr()).parent.unwrap();
            if let Some(parent) = (*node_p.as_ptr()).parent {
                let mut node_p_from_left = false;
                if let Some(p_node_p) = (*parent.as_ptr()).left
                    && std::ptr::eq(p_node_p.as_ptr(), node_p.as_ptr())
                {
                    node_p_from_left = true;
                }
                let node_p = if node_p_from_left {
                    (*parent.as_ptr()).left.unwrap()
                } else {
                    (*parent.as_ptr()).right.unwrap()
                };
                (*node_a.as_ptr()).is_red = false;
                let node_c = (*node_a.as_ptr()).right.unwrap();
                (*node_c.as_ptr()).is_red = false;
                let node_e = (*node_c.as_ptr()).left;
                let node_d = (*node_c.as_ptr()).right;
                (*node_c.as_ptr()).is_red = true;
                (*node_c.as_ptr()).parent = Some(parent);
                (*node_c.as_ptr()).left = Some(node_a);
                (*node_a.as_ptr()).parent = Some(node_c);
                (*node_c.as_ptr()).right = Some(node_p);
                (*node_p.as_ptr()).parent = Some(node_c);
                (*node_p.as_ptr()).left = node_d;
                if let Some(n_d) = node_d {
                    (*n_d.as_ptr()).parent = Some(node_p);
                }
                (*node_a.as_ptr()).right = node_e;
                if let Some(n_e) = node_e {
                    (*n_e.as_ptr()).parent = Some(node_a);
                }
                if node_p_from_left {
                    (*parent.as_ptr()).left = Some(node_c);
                } else {
                    (*parent.as_ptr()).right = Some(node_c);
                }
                Some(parent)
            } else {
                let fixed_head = self.fixed_head.unwrap();
                let node_p = (*fixed_head.as_ptr()).left.unwrap();

                let node_a = (*node_p.as_ptr()).left.unwrap();
                (*node_a.as_ptr()).is_red = false;
                let node_c = (*node_a.as_ptr()).right.unwrap();
                (*node_c.as_ptr()).is_red = false;
                let node_e = (*node_c.as_ptr()).left;
                let node_d = (*node_c.as_ptr()).right;
                (*node_c.as_ptr()).is_red = false;
                (*node_c.as_ptr()).parent = None;
                (*node_c.as_ptr()).left = Some(node_a);
                (*node_a.as_ptr()).parent = Some(node_c);
                (*node_c.as_ptr()).right = Some(node_p);
                (*node_p.as_ptr()).parent = Some(node_c);
                (*node_p.as_ptr()).left = node_d;
                if let Some(n_d) = node_d {
                    (*n_d.as_ptr()).parent = Some(node_p);
                }
                (*node_a.as_ptr()).right = node_e;
                if let Some(n_e) = node_e {
                    (*n_e.as_ptr()).parent = Some(node_a);
                }
                (*fixed_head.as_ptr()).left = Some(node_c);
                (*fixed_head.as_ptr()).left
            }
        }
    }

    /// Rotate left without parent
    /// A is root
    ///
    /// ```text
    ///             A                  C
    ///            / \\               //\
    ///           B    C     =>      A   D
    ///               / \           / \
    ///              E   D         B   E
    ///
    /// ```
    ///
    /// Rotate left  with parent
    ///
    /// ```text
    ///             P                  P
    ///             |                  |
    ///             A                  C
    ///            / \\               //\
    ///           B    C     =>      A   D
    ///               / \           / \
    ///              E   D         B   E
    ///
    /// ```
    unsafe fn rotate_left(&mut self, node_a: NonNull<Node<T>>) -> Link<T> {
        unsafe {
            if let Some(parent) = (*node_a.as_ptr()).parent {
                let mut node_a_from_left = false;
                if let Some(p_node_a) = (*parent.as_ptr()).left
                    && std::ptr::eq(p_node_a.as_ptr(), node_a.as_ptr())
                {
                    node_a_from_left = true;
                }
                let node_a = if node_a_from_left {
                    (*parent.as_ptr()).left.unwrap()
                } else {
                    (*parent.as_ptr()).right.unwrap()
                };
                let node_c = (*node_a.as_ptr()).right.unwrap();
                (*node_c.as_ptr()).is_red = (*node_a.as_ptr()).is_red;
                (*node_a.as_ptr()).is_red = true;
                (*node_c.as_ptr()).parent = Some(parent);
                if node_c.as_ref().left.is_some() {
                    let mut node_e = (*node_c.as_ptr()).left;
                    if let Some(e) = &mut node_e {
                        (*e.as_ptr()).parent = Some(node_a);
                    }
                    (*node_a.as_ptr()).right = node_e;
                } else {
                    (*node_a.as_ptr()).right = None;
                }
                (*node_c.as_ptr()).left = Some(node_a);
                (*node_a.as_ptr()).parent = Some(node_c);
                if node_a_from_left {
                    (*parent.as_ptr()).left = Some(node_c);
                    (*parent.as_ptr()).left
                } else {
                    (*parent.as_ptr()).right = Some(node_c);
                    (*parent.as_ptr()).right
                }
            } else {
                let fixed_head = self.fixed_head.unwrap();
                let node_a = (*fixed_head.as_ptr()).left.unwrap();
                let node_c = (*node_a.as_ptr()).right.unwrap();
                (*node_c.as_ptr()).is_red = (*node_a.as_ptr()).is_red;
                (*node_a.as_ptr()).is_red = true;
                if node_c.as_ref().left.is_some() {
                    let mut node_e = (*node_c.as_ptr()).left;
                    if let Some(e) = &mut node_e {
                        (*e.as_ptr()).parent = Some(node_a);
                    }
                    (*node_a.as_ptr()).right = node_e;
                } else {
                    (*node_a.as_ptr()).right = None;
                }
                (*node_c.as_ptr()).left = Some(node_a);
                (*node_a.as_ptr()).parent = Some(node_c);
                (*node_c.as_ptr()).parent = None;
                (*fixed_head.as_ptr()).left = Some(node_c);
                (*fixed_head.as_ptr()).left
            }
        }
    }

    /// Rotate right without parent
    /// A is root
    ///
    /// ```text
    ///              A               B
    ///            // \             / \\
    ///            B   C     =>    E    A
    ///          // \                  / \
    ///         E    D                D   C
    /// ```
    ///
    /// Rotate right with parent
    ///
    /// ```text
    ///                P               P
    ///                |               |
    ///                A               B
    ///              // \             / \\
    ///              B   C     =>    E    A
    ///            // \                  / \
    ///           E    D                D   C
    ///
    /// ```
    unsafe fn rotate_right(&mut self, node_a: NonNull<Node<T>>) -> Link<T> {
        unsafe {
            if let Some(parent) = (*node_a.as_ptr()).parent {
                let mut node_a_from_left = false;
                if let Some(p_node_a) = (*parent.as_ptr()).left
                    && std::ptr::eq(p_node_a.as_ptr(), node_a.as_ptr())
                {
                    node_a_from_left = true;
                }
                let node_a = if node_a_from_left {
                    (*parent.as_ptr()).left.unwrap()
                } else {
                    (*parent.as_ptr()).right.unwrap()
                };
                let node_b = (*node_a.as_ptr()).left.unwrap();
                (*node_b.as_ptr()).parent = Some(parent);
                (*node_b.as_ptr()).is_red = (*node_a.as_ptr()).is_red;
                (*node_a.as_ptr()).is_red = true;
                if node_b.as_ref().right.is_some() {
                    let mut node_d = (*node_b.as_ptr()).right;
                    if let Some(d) = &mut node_d {
                        (*d.as_ptr()).parent = Some(node_a);
                    }
                    (*node_a.as_ptr()).left = node_d;
                } else {
                    (*node_a.as_ptr()).left = None;
                }
                (*node_b.as_ptr()).right = Some(node_a);
                (*node_a.as_ptr()).parent = Some(node_b);
                if node_a_from_left {
                    (*parent.as_ptr()).left = Some(node_b);
                    (*parent.as_ptr()).left
                } else {
                    (*parent.as_ptr()).right = Some(node_b);
                    (*parent.as_ptr()).right
                }
            } else {
                let fixed_head = self.fixed_head.unwrap();
                let node_a = (*fixed_head.as_ptr()).left.unwrap();
                let node_b = (*node_a.as_ptr()).left.unwrap();
                (*node_b.as_ptr()).is_red = (*node_a.as_ptr()).is_red;
                (*node_a.as_ptr()).is_red = true;
                if node_b.as_ref().right.is_some() {
                    let mut node_d = (*node_b.as_ptr()).right;
                    if let Some(d) = &mut node_d {
                        (*d.as_ptr()).parent = Some(node_a);
                    }
                    (*node_a.as_ptr()).left = node_d;
                } else {
                    (*node_a.as_ptr()).left = None;
                }
                (*node_b.as_ptr()).right = Some(node_a);
                (*node_a.as_ptr()).parent = Some(node_b);
                (*node_b.as_ptr()).parent = None;
                (*fixed_head.as_ptr()).left = Some(node_b);
                (*fixed_head.as_ptr()).left
            }
        }
    }

    /// Flip color
    ///
    /// ```text
    ///             |          ||
    ///             A    =>    A
    ///           // \\       / \
    ///          B    C      B   C
    ///
    /// ```
    unsafe fn flip_colors(&mut self, mut node: NonNull<Node<T>>) {
        unsafe {
            if node.as_ref().left.is_some() && node.as_ref().right.is_some() {
                if let Some(ref mut left) = node.as_mut().left {
                    left.as_mut().is_red = false;
                }
                if let Some(ref mut right) = node.as_mut().right {
                    right.as_mut().is_red = false;
                }
                node.as_mut().is_red = true;

                if (*node.as_ptr()).parent.is_none() {
                    node.as_mut().is_red = false;
                }
            }
        }
    }

    unsafe fn check_put_balancing(&mut self, node: NonNull<Node<T>>) -> OperationPut {
        unsafe {
            if node.as_ref().right.is_some() {
                let r = node.as_ref().right.unwrap();
                if node.as_ref().left.is_some() && !node.as_ref().is_red {
                    let l = node.as_ref().left.unwrap();
                    if l.as_ref().is_red && r.as_ref().is_red {
                        return OperationPut::FlipColors;
                    }
                }
                if r.as_ref().is_red {
                    return OperationPut::Left;
                }
            }
            if node.as_ref().is_red && node.as_ref().left.is_some() {
                let l = node.as_ref().left.unwrap();
                if l.as_ref().is_red {
                    return OperationPut::Right;
                }
            }
            OperationPut::Nothing
        }
    }

    unsafe fn put_balancing(&mut self, next: NonNull<Node<T>>) {
        unsafe {
            let mut next = next;
            loop {
                match self.check_put_balancing(next) {
                    OperationPut::Left => {
                        if (*next.as_ptr()).is_red {
                            if let Some(n) = self.rotate_left_right_flip_color(next) {
                                next = n;
                            }
                        } else {
                            if let Some(n) = self.rotate_left(next) {
                                next = n;
                            }
                        }
                    }
                    OperationPut::Right => {
                        if (*next.as_ptr()).parent.is_some() {
                            let node_a = (*next.as_ptr()).parent.unwrap();
                            if let Some(n) = self.rotate_right(node_a) {
                                next = n;
                            }
                        }
                    }
                    OperationPut::FlipColors => {
                        self.flip_colors(next);
                        if (*next.as_ptr()).parent.is_some() {
                            next = (*next.as_ptr()).parent.unwrap();
                        } else {
                            break;
                        }
                    }
                    OperationPut::Nothing => {
                        break;
                    }
                }
            }
        }
    }

    unsafe fn find_put_parent_candidate(&mut self, parent: Link<T>, elem: &T) -> Link<T> {
        unsafe {
            if let Some(parent) = parent {
                match elem.cmp(&parent.as_ref().value) {
                    Ordering::Equal => None,
                    Ordering::Less => {
                        if (*parent.as_ptr()).left.is_some() {
                            self.find_put_parent_candidate((*parent.as_ptr()).left, elem)
                        } else {
                            Some(parent)
                        }
                    }
                    Ordering::Greater => {
                        if (*parent.as_ptr()).right.is_some() {
                            self.find_put_parent_candidate((*parent.as_ptr()).right, elem)
                        } else {
                            Some(parent)
                        }
                    }
                }
            } else {
                parent
            }
        }
    }

    unsafe fn attach_node(&mut self, parent: NonNull<Node<T>>, elem: T) -> bool {
        unsafe {
            match elem.cmp(&parent.as_ref().value) {
                Ordering::Equal => false,
                Ordering::Less => {
                    (*parent.as_ptr()).left = Node::new_red(elem, parent);
                    true
                }
                Ordering::Greater => {
                    (*parent.as_ptr()).right = Node::new_red(elem, parent);
                    true
                }
            }
        }
    }

    /// Returns the default iterator over the tree.
    ///
    /// The default traversal is in-order (`LNR`), so values are yielded in
    /// ascending order.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbtree_extended::RedBlackTree;
    ///
    /// let mut tree = RedBlackTree::new();
    /// tree.extend([3, 1, 2]);
    ///
    /// let values: Vec<i32> = tree.iter().copied().collect();
    ///
    /// assert_eq!(values, vec![1, 2, 3]);
    /// ```
    #[cfg(feature = "in-order")]
    pub fn iter(&self) -> IterInOrder<'_, T> {
        self.iter_inorder()
    }

    /// Returns an in-order (`LNR`) iterator.
    ///
    /// Values are yielded in ascending order.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbtree_extended::RedBlackTree;
    ///
    /// let mut tree = RedBlackTree::new();
    /// tree.extend([3, 1, 2]);
    ///
    /// assert_eq!(
    ///     tree.iter_inorder().copied().collect::<Vec<_>>(),
    ///     vec![1, 2, 3]
    /// );
    /// ```
    #[cfg(feature = "in-order")]
    pub fn iter_inorder(&self) -> IterInOrder<'_, T> {
        unsafe { IterInOrder::new(leftmost_child_in_order(self.get_root()), self.count) }
    }

    /// Returns a reverse in-order (`RNL`) iterator.
    ///
    /// Values are yielded in descending order.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbtree_extended::RedBlackTree;
    ///
    /// let mut tree = RedBlackTree::new();
    /// tree.extend([3, 1, 2]);
    ///
    /// assert_eq!(
    ///     tree.iter_reverse_inorder().copied().collect::<Vec<_>>(),
    ///     vec![3, 2, 1]
    /// );
    /// ```
    #[cfg(feature = "reverse-in-order")]
    pub fn iter_reverse_inorder(&self) -> IterReverseInOrder<'_, T> {
        unsafe {
            IterReverseInOrder::new(
                rightmost_child_reverse_in_order(self.get_root()),
                self.count,
            )
        }
    }

    /// Returns a pre-order (`NLR`) iterator.
    ///
    /// The current node is visited before its left and right subtrees.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbtree_extended::RedBlackTree;
    ///
    /// let mut tree = RedBlackTree::new();
    /// tree.extend([4, 2, 6, 1, 3, 5, 7]);
    ///
    /// let values: Vec<i32> = tree.iter_preorder().copied().collect();
    ///
    /// assert_eq!(values.len(), tree.len());
    /// ```
    #[cfg(feature = "pre-order")]
    pub fn iter_preorder(&self) -> IterPreOrder<'_, T> {
        IterPreOrder::new(self.get_root(), self.count)
    }

    /// Returns a post-order (`LRN`) iterator.
    ///
    /// The left and right subtrees are visited before the current node.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbtree_extended::RedBlackTree;
    ///
    /// let mut tree = RedBlackTree::new();
    /// tree.extend([4, 2, 6, 1, 3, 5, 7]);
    ///
    /// let values: Vec<i32> = tree.iter_postorder().copied().collect();
    ///
    /// assert_eq!(values.len(), tree.len());
    /// ```
    #[cfg(feature = "post-order")]
    pub fn iter_postorder(&self) -> IterPostOrder<'_, T> {
        unsafe {
            let first = self.get_root().map(|root| leaf_post_order(root));
            IterPostOrder::new(first, self.count)
        }
    }

    /// Returns all values in level-order traversal.
    ///
    /// Nodes are visited level by level, starting from the root.
    ///
    /// # Examples
    ///
    /// ```
    /// use rbtree_extended::RedBlackTree;
    ///
    /// let mut tree = RedBlackTree::new();
    /// tree.extend([4, 2, 6, 1, 3, 5, 7]);
    ///
    /// let values = tree.iter_level_order();
    ///
    /// assert_eq!(values.len(), tree.len());
    /// ```
    #[cfg(feature = "bfs")]
    pub fn iter_level_order(&self) -> Vec<&T> {
        let mut ret: Vec<&T> = vec![];
        if let Some(root) = self.get_root() {
            iter_level_order_impl(root, &mut ret);
        }
        ret
    }
}

#[cfg(feature = "in-order")]
impl<'a, T: Ord + Default> std::iter::IntoIterator for &'a RedBlackTree<T> {
    type IntoIter = IterInOrder<'a, T>;
    type Item = &'a T;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: Ord + Default> Drop for RedBlackTree<T> {
    fn drop(&mut self) {
        unsafe {
            if let Some(fixed_head) = self.fixed_head {
                self.remove_tree((*fixed_head.as_ptr()).left);
                let _ = Box::from_raw(fixed_head.as_ptr());
            }
            self.fixed_head = None;
        }
    }
}

fn find_node<T: Ord>(fromnode: Link<T>, value: &T) -> Link<T> {
    unsafe {
        if let Some(fromnode) = fromnode {
            match value.cmp(&(*fromnode.as_ptr()).value) {
                Ordering::Equal => Some(fromnode),
                Ordering::Less => find_node((*fromnode.as_ptr()).left, value),
                Ordering::Greater => find_node((*fromnode.as_ptr()).right, value),
            }
        } else {
            fromnode
        }
    }
}

fn helper_display_tree<T: Display>(node: NonNull<Node<T>>) -> String {
    unsafe {
        let mut s: String = "".into();
        let color = if (*node.as_ptr()).is_red {
            "[color=\"red\"]"
        } else {
            "[color=\"black\"]"
        };

        if let Some(left) = (*node.as_ptr()).left {
            s.push_str(&format!(
                "\t{n1}->{n2} {color1}; {n1} {color1}; {n2} {color2};\n",
                n1 = (*node.as_ptr()).value,
                n2 = (*left.as_ptr()).value,
                color1 = color,
                color2 = if (*left.as_ptr()).is_red {
                    "[color=\"red\"]"
                } else {
                    "[color=\"black\"]"
                },
            ));
            s.push_str(&helper_display_tree(left));
        } else if (*node.as_ptr()).right.is_some() {
            s.push_str(&format!(
                "\t{n1}->node_null_{n1} [color=\"grey\"]; {n1} {color1};\n",
                n1 = (*node.as_ptr()).value,
                color1 = color
            ));
            s.push_str(&format!(
                "\tnode_null_{n1}[label=\"null\"]\n",
                n1 = (*node.as_ptr()).value
            ));
        }

        if let Some(right) = (*node.as_ptr()).right {
            s.push_str(&format!(
                "\t{n1}->{n2} {color1}; {n1} {color1}; {n2} {color2};\n",
                n1 = (*node.as_ptr()).value,
                n2 = (*right.as_ptr()).value,
                color2 = if (*right.as_ptr()).is_red {
                    "[color=\"red\"]"
                } else {
                    "[color=\"black\"]"
                },
                color1 = color
            ));
            s.push_str(&helper_display_tree(right));
        } else {
            s.push_str(&format!(
                "\t{n1}->node_null_{n1} [color=\"grey\"]; {n1} {color1};\n",
                n1 = (*node.as_ptr()).value,
                color1 = color
            ));
            s.push_str(&format!(
                "\tnode_null_{}[label=\"null\"]\n",
                (*node.as_ptr()).value
            ));
        }
        s
    }
}

impl<T: Ord + Default> Default for RedBlackTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "in-order")]
impl<T: Ord + Default + Clone> Clone for RedBlackTree<T> {
    fn clone(&self) -> Self {
        let mut new_list = Self::new();
        for value in self {
            new_list.insert(value.clone());
        }
        new_list
    }
}

#[cfg(feature = "in-order")]
impl<T: Ord + Default> Extend<T> for RedBlackTree<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for value in iter {
            self.insert(value);
        }
    }
}

#[cfg(feature = "in-order")]
impl<T: Ord + Default> FromIterator<T> for RedBlackTree<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        list.extend(iter);
        list
    }
}

#[cfg(feature = "in-order")]
impl<T: Ord + Default + Debug> Debug for RedBlackTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

#[cfg(feature = "in-order")]
impl<T: Ord + Default> PartialEq for RedBlackTree<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter_inorder().eq(other.iter_inorder())
    }
}

#[cfg(feature = "in-order")]
impl<T: Ord + Default> Eq for RedBlackTree<T> {}

#[cfg(feature = "in-order")]
impl<T: Ord + Default> PartialOrd for RedBlackTree<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(feature = "in-order")]
impl<T: Ord + Default> Ord for RedBlackTree<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.iter_inorder().cmp(other.iter_inorder())
    }
}

#[cfg(feature = "in-order")]
impl<T: Ord + Default + std::hash::Hash> std::hash::Hash for RedBlackTree<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.len().hash(state);
        for item in self {
            item.hash(state);
        }
    }
}
