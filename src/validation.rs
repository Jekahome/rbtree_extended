use crate::node::Link;
use crate::tree::RedBlackTree;

impl<T: Ord + Default> RedBlackTree<T> {
    // red-red violations,
    // min black-height,
    // max black-height,
    // BST ordering,
    // parent links,
    // reachable node count
    unsafe fn validate(
        node: &Link<T>,
        expected_parent: Link<T>,
        parent_is_red: bool,
        black_height: usize,
        min: Option<&T>,
        max: Option<&T>,
    ) -> (usize, usize, usize, bool, bool, usize) {
        unsafe {
            if let Some(n) = node {
                let value = &n.as_ref().value;

                // 1. Check parent
                let parent_valid = match (n.as_ref().parent, expected_parent) {
                    (None, None) => true,

                    (Some(actual), Some(expected)) => {
                        std::ptr::eq(actual.as_ptr(), expected.as_ptr())
                    }

                    _ => false,
                };

                // 2. Check BST ordering
                if let Some(min) = min
                    && value <= min
                {
                    return (0, 0, 0, false, parent_valid, 1);
                }

                if let Some(max) = max
                    && value >= max
                {
                    return (0, 0, 0, false, parent_valid, 1);
                }

                let is_red = n.as_ref().is_red;

                // 3. A red node cannot have a red parent.
                let red_red = if parent_is_red && is_red { 1 } else { 0 };

                // 4. Black height
                let black_height = if is_red {
                    black_height
                } else {
                    black_height + 1
                };

                let left = Self::validate(
                    &n.as_ref().left,
                    Some(*n),
                    is_red,
                    black_height,
                    min,
                    Some(value),
                );

                let right = Self::validate(
                    &n.as_ref().right,
                    Some(*n),
                    is_red,
                    black_height,
                    Some(value),
                    max,
                );

                (
                    red_red + left.0 + right.0,
                    std::cmp::min(left.1, right.1),
                    std::cmp::max(left.2, right.2),
                    left.3 && right.3,
                    parent_valid && left.4 && right.4,
                    1 + left.5 + right.5,
                )
            } else {
                (0, black_height, black_height, true, true, 0)
            }
        }
    }

    pub fn is_valid(&self) -> bool {
        unsafe {
            // tree is empty
            if self.count == 0 {
                return match self.fixed_head {
                    None => true,

                    Some(fixed_head) => (*fixed_head.as_ptr()).left.is_none(),
                };
            }

            // count > 0, so sentinel must exist.
            let Some(fixed_head) = self.fixed_head else {
                return false;
            };

            // And root must exist.
            let root_link = (*fixed_head.as_ptr()).left;

            let Some(root) = root_link else {
                return false;
            };

            // Root is always BLACK
            if root.as_ref().is_red {
                return false;
            }

            // Root has no parent
            if root.as_ref().parent.is_some() {
                return false;
            }

            let result = Self::validate(&root_link, None, false, 0, None, None);

            let red_red = result.0;
            let black_height_min = result.1;
            let black_height_max = result.2;
            let bst_valid = result.3;
            let parents_valid = result.4;
            let reachable_nodes = result.5;

            red_red == 0
                && black_height_min == black_height_max
                && bst_valid
                && parents_valid
                && reachable_nodes == self.count
        }
    }
}
