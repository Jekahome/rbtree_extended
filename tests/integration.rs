use rbtree_extended::RedBlackTree;

#[cfg(test)]
mod tests {
    use super::*;

    // $ cargo test remove_only_root -- --nocapture
    #[test]
    fn remove_only_root() {
        let mut tree = RedBlackTree::new();

        tree.insert(10);

        assert!(tree.remove(&10));
        assert_eq!(tree.len(), 0);
        assert!(!tree.contains(&10));

        assert!(tree.insert(20));
        assert!(tree.contains(&20));
        assert_eq!(tree.len(), 1);
    }

    // $ cargo test test_success -- --nocapture
    #[test]
    fn test_success() {
        let mut tree: RedBlackTree<i32> = RedBlackTree::new();
        let nodes = vec![
            480, 978, 379, 784, 999, 695, 23, 97, 309, 312, 449, 958, 992, 220, 95, 257, 869, 959,
            450, 258, 315, 783, 731, 914, 880, 984, 734, 570, 801, 908, 181, 466, 238, 916, 77,
            801, 867, 382, 943, 603, 65, 545, 200, 759, 158, 987, 821, 630, 537, 704, 149, 617,
            498, 261, 160, 192, 760, 417, 939, 757, 858, 376, 885, 336, 764, 443, 155, 983, 586,
            957, 375, 893, 707, 255, 811, 86, 370, 384, 177, 834, 177, 834, 313, 209, 623, 176,
            875, 748, 949, 529, 932, 369, 385, 419, 222, 719, 342, 68, 156, 314, 343, 262, 467,
            499, 604, 732, 758, 765, 812, 859, 876,
        ];

        for i in nodes {
            tree.insert(i);
        }

        println!("{}", tree.to_dot());

        assert!(tree.is_valid());
    }

    // $ cargo test test_rotate_right_success -- --nocapture
    #[test]
    fn test_rotate_right_success() {
        let mut tree: RedBlackTree<i32> = RedBlackTree::new();
        let nodes = vec![575, 396, 139, 792, 546, 73, 7];
        for i in nodes {
            tree.insert(i);
        }

        assert!(tree.is_valid());
    }

    // $ cargo test test_find_success -- --nocapture
    #[test]
    fn test_find_success() {
        let mut tree = RedBlackTree::new();
        let nodes = 0..=28;
        for i in nodes {
            tree.insert(i);
        }
        assert!(tree.contains(&4));
    }

    // $ cargo test test_validate_success -- --nocapture
    #[test]
    fn test_validate_success() {
        let mut tree = RedBlackTree::new();
        let nodes = 0..=28;
        for i in nodes {
            tree.insert(i);
        }
        assert!(tree.is_valid());
    }

    // $ cargo test test_remove_tree_success -- --nocapture
    #[test]
    #[should_panic(expected = "tree empty")]
    fn test_remove_tree_success() {
        let mut tree = RedBlackTree::new();
        let nodes = 1..=6;
        for i in nodes {
            tree.insert(i);
        }
        assert_eq!(6, tree.len());
        assert_eq!(0, tree.len(), "tree empty");
    }

    // $ cargo test test_remove_1_0_0_success -- --nocapture
    #[test]
    fn test_remove_1_0_0_success() {
        let mut tree = RedBlackTree::new();
        let nodes = 0..=27;
        for i in nodes {
            tree.insert(i);
        }
        assert!(tree.is_valid());
        tree.remove(&26);
        assert!(tree.is_valid());
    }

    // $ cargo test test_remove_3_0_0_success -- --nocapture
    #[test]
    fn test_remove_3_0_0_success() {
        let mut tree = RedBlackTree::new();
        let nodes = 0..=3;
        for i in nodes {
            tree.insert(i);
        }
        assert!(tree.is_valid());
        tree.remove(&3);
        assert!(tree.is_valid());
    }

    // $ cargo test test_remove_node_4_0_0_to_3_0_0_success -- --nocapture
    #[test]
    fn test_remove_node_4_0_0_to_3_0_0_success() {
        let mut tree = RedBlackTree::new();
        let nodes = 0..=9;
        for i in nodes {
            tree.insert(i);
        }
        assert!(tree.is_valid());
        tree.remove(&7);
        assert!(tree.is_valid());
    }

    // $ cargo test test_remove_node_4_0_0_to_1_0_0_success -- --nocapture
    #[test]
    fn test_remove_node_4_0_0_to_1_0_0_success() {
        let mut tree = RedBlackTree::new();
        let nodes = vec![315, 897, 267, 995, 843, 520];
        for i in nodes {
            tree.insert(i);
        }
        assert!(tree.is_valid());
        tree.remove(&897);
        assert!(tree.is_valid());
    }

    // $ cargo test test_remove_black_2_1_1_1_success  -- --nocapture
    #[test]
    fn test_remove_black_2_1_1_1_success() {
        let mut tree = RedBlackTree::new();
        let nodes = vec![314, 147, 119, 331, 755, 449, 118];
        for i in nodes {
            tree.insert(i);
        }
        assert!(tree.is_valid());

        tree.remove(&314); //2.1.1.1

        assert!(tree.is_valid());
        tree.remove(&147); // 2.1.2.1
        assert!(tree.is_valid());
    }

    // $ cargo test test_remove_black_2_1_1_2_success  -- --nocapture
    #[test]
    fn test_remove_black_2_1_1_2_success() {
        let mut tree = RedBlackTree::new();
        let nodes = vec![231, 511, 914, 699, 532, 531];
        for i in nodes {
            tree.insert(i);
        }
        assert!(tree.is_valid());
        tree.remove(&231);
        assert!(tree.is_valid());
    }

    // $ cargo test test_remove_black_2_1_2_1_success  -- --nocapture
    #[test]
    fn test_remove_black_2_1_2_1_success() {
        let mut tree = RedBlackTree::new();
        let nodes = vec![438, 440, 260, 530, 34, 355];
        for i in nodes {
            tree.insert(i);
        }
        assert!(tree.is_valid());
        tree.remove(&355);
        assert!(tree.is_valid());
    }

    // $ cargo test test_remove_black_2_1_2_2_success  -- --nocapture
    #[test]
    fn test_remove_black_2_1_2_2_success() {
        let mut tree = RedBlackTree::new();
        let nodes = vec![231, 511, 914, 699, 532];
        for i in nodes {
            tree.insert(i);
        }
        assert!(tree.is_valid());
        println!("{}", tree.to_dot());
        tree.remove(&231);
        assert!(tree.is_valid());
    }

    // $ cargo test test_remove_black_2_2_1_node_a_root_success -- --nocapture
    #[test]
    fn test_remove_black_2_2_1_node_a_root_success() {
        let mut tree = RedBlackTree::new();
        let nodes = vec![315, 897, 267, 995, 843, 520];
        for i in nodes {
            tree.insert(i);
        }
        assert!(tree.is_valid());
        tree.remove(&995);
        assert!(tree.is_valid());
    }

    // $ cargo test test_remove_black_2_2_1_success -- --nocapture
    #[test]
    fn test_remove_black_2_2_1_success() {
        let mut tree = RedBlackTree::new();
        let nodes = vec![486, 226, 612, 121, 479, 69, 559, 990, 290, 324, 280];
        for i in nodes {
            tree.insert(i);
        }
        assert!(tree.is_valid());
        tree.remove(&479);
        assert!(!tree.contains(&479));
        assert!(tree.is_valid());
    }

    // $ cargo test test_remove_black_2_2_2_success -- --nocapture
    #[test]
    fn test_remove_black_2_2_2_success() {
        let mut tree = RedBlackTree::new();
        let nodes: Vec<i32> = vec![119, 331, 755, 449, 118, 850, 495, 382, 328];
        for i in nodes {
            tree.insert(i);
        }
        assert!(tree.is_valid());
        tree.remove(&382);
        assert!(tree.is_valid());
    }

    // $ cargo test test_remove_black_2_2_2_root_success -- --nocapture
    #[test]
    fn test_remove_black_2_2_2_root_success() {
        let mut tree = RedBlackTree::new();
        let nodes: Vec<i32> = vec![106, 734, 951, 753, 205, 730];
        for i in nodes {
            tree.insert(i);
        }
        tree.remove(&753);
        assert!(tree.is_valid());
        tree.remove(&951);
        assert!(tree.is_valid());
    }

    // $ cargo test test_remove_black_2_3_1_1_success -- --nocapture
    #[test]
    fn test_remove_black_2_3_1_1_success() {
        let mut tree = RedBlackTree::new();
        let nodes: Vec<i32> = vec![575, 396, 139, 792, 546, 73, 7, 6];
        for i in nodes {
            tree.insert(i);
        }
        assert!(tree.is_valid());
        tree.remove(&139);
        assert!(tree.is_valid());
    }

    // $ cargo test test_remove_black_2_3_1_1_root_success -- --nocapture
    #[test]
    fn test_remove_black_2_3_1_1_root_success() {
        let mut tree = RedBlackTree::new();
        let nodes: Vec<i32> = vec![106, 107, 108, 105];
        for i in nodes {
            tree.insert(i);
        }
        assert!(tree.is_valid());
        tree.remove(&108);
        assert!(tree.is_valid());
    }

    // $ cargo test test_remove_black_2_3_1_2_success -- --nocapture
    #[test]
    fn test_remove_black_2_3_1_2_success() {
        let mut tree = RedBlackTree::new();
        let nodes: Vec<i32> = vec![575, 396, 139, 792, 546, 73, 7, 138];
        for i in nodes {
            tree.insert(i);
        }
        assert!(tree.is_valid());
        tree.remove(&7);
        assert!(tree.is_valid());
    }

    // $ cargo test test_remove_black_2_3_1_2_root_success -- --nocapture
    #[test]
    fn test_remove_black_2_3_1_2_root_success() {
        let mut tree = RedBlackTree::new();
        let nodes: Vec<i32> = vec![106, 107, 109, 108];
        for i in nodes {
            tree.insert(i);
        }
        assert!(tree.is_valid());
        tree.remove(&106);
        assert!(tree.is_valid());
    }

    // $ cargo test test_remove_black_2_3_2_1_success -- --nocapture
    #[test]
    fn test_remove_black_2_3_2_1_success() {
        let mut tree = RedBlackTree::new();
        let nodes: Vec<i32> = vec![15, 19, 20, 14, 17, 18, 16];
        for i in nodes {
            tree.insert(i);
        }
        assert!(tree.is_valid());
        tree.remove(&20);
        assert!(tree.is_valid());
    }

    // $ cargo test test_remove_black_2_3_2_1_root_success -- --nocapture
    #[test]
    fn test_remove_black_2_3_2_1_root_success() {
        let mut tree = RedBlackTree::new();
        let nodes: Vec<i32> = vec![5, 4, 6];
        for i in nodes {
            tree.insert(i);
        }
        assert!(tree.is_valid());
        tree.remove(&6);
        assert!(tree.is_valid());
        println!("{}", tree.to_dot());
    }

    // $ cargo test test_remove_black_2_3_2_2_up_to_down_success -- --nocapture
    #[test]
    fn test_remove_black_2_3_2_2_up_to_down_success() {
        let mut tree: RedBlackTree<i32> = RedBlackTree::new();

        let nodes = vec![
            352, 873, 462, 836, 316, 381, 595, 288, 600, 263, 310, 74, 544, 621, 402, 618, 61, 576,
            654, 579, 985, 949, 856, 796, 894, 6, 991, 880, 652, 349, 525, 9, 515, 371, 53,
        ];
        for i in nodes {
            tree.insert(i);
        }

        println!("{}", tree.to_dot());

        tree.remove(&6);
        println!("{}", tree.to_dot());
        assert!(tree.is_valid());
    }

    // $ cargo test test_remove_black_2_3_2_2_start_v5_success -- --nocapture
    #[test]
    fn test_remove_black_2_3_2_2_start_v5_success() {
        let mut tree: RedBlackTree<i32> = RedBlackTree::new();

        let nodes = vec![
            480, 978, 379, 784, 398, 71, 695, 23, 97, 309, 312, 449, 958, 992, 220, 95, 257, 869,
            959, 450, 258, 315, 783, 731, 914, 880, 984, 734, 570, 801, 908, 181, 466, 238, 916,
            77, 801, 867, 382, 943, 603, 65, 545, 200, 759, 158, 987, 821, 630, 537, 704, 149, 617,
            498, 261, 160, 192, 760, 417, 939, 757, 858, 376, 885, 336, 764, 443, 155, 983, 586,
            957, 375, 893, 707, 255, 811, 86, 370, 384, 177, 834, 177, 834, 313, 209, 623, 176,
            875, 748, 949, 529, 932, 369, 385, 419, 222, 719, 342, 68, 156, 314, 343, 262, 467,
            499, 604, 732, 758, 765, 812, 859, 876,
        ];
        for i in nodes {
            tree.insert(i);
        }

        tree.remove(&23);

        assert!(tree.is_valid());
    }

    // $ cargo test test_remove_black_2_3_2_2_success -- --nocapture
    #[test]
    fn test_remove_black_2_3_2_2_success() {
        let mut tree = RedBlackTree::new();
        let nodes: Vec<i32> = vec![246, 562, 950, 237, 417, 418, 416];
        for i in nodes {
            tree.insert(i);
        }
        assert!(tree.is_valid());
        tree.remove(&418);
        assert!(tree.is_valid());
    }

    // $ cargo test test_remove_black_2_3_2_2_root_success -- --nocapture
    #[test]
    fn test_remove_black_2_3_2_2_root_success() {
        let mut tree = RedBlackTree::new();
        let nodes: Vec<i32> = vec![5, 4, 6];
        for i in nodes {
            tree.insert(i);
        }
        assert!(tree.is_valid());
        tree.remove(&4);
        assert!(tree.is_valid());
    }

    /*
             4
           /   \
          2     9
         / \   //\
        1   3  7   10
              / \
             6   8

        Depth-First Search Pre-order:  4, 2, 1, 3, 9, 7, 6, 8, 10
        Depth-First Search In-order:   1, 2, 3, 4, 6, 7, 8, 9, 10
        Depth-First Search Post-order: 1, 3, 2, 6, 8, 7, 10, 9, 4
        Breadth-First Search:          4, 2, 9, 1, 3, 7, 10, 6, 8
    */
    // $ cargo test test_dfs_iter_in_order -- --nocapture
    #[cfg(feature = "in-order")]
    #[test]
    fn test_dfs_iter_in_order() {
        let mut tree: RedBlackTree<i32> = RedBlackTree::new();
        let nodes: Vec<i32> = vec![4, 3, 9, 1, 2, 10, 7, 8, 6];
        for i in nodes {
            tree.insert(i);
        }

        let mut buf: Vec<&i32> = vec![];
        for item in tree.iter_inorder() {
            buf.push(item);
        }

        assert_eq!(buf, vec![&1, &2, &3, &4, &6, &7, &8, &9, &10]);

        assert_eq!(Some(&6), tree.successor_dfs_in_order(4), "4->6");
        assert_eq!(Some(&2), tree.successor_dfs_in_order(1), "1->2");
        assert_eq!(Some(&3), tree.successor_dfs_in_order(2), "2->3");
        assert_eq!(Some(&4), tree.successor_dfs_in_order(3), "3->4");
        assert_eq!(Some(&6), tree.successor_dfs_in_order(4), "4->6");
        assert_eq!(Some(&7), tree.successor_dfs_in_order(6), "6->7");
        assert_eq!(Some(&8), tree.successor_dfs_in_order(7), "7->8");
        assert_eq!(Some(&9), tree.successor_dfs_in_order(8), "8->9");
        assert_eq!(Some(&10), tree.successor_dfs_in_order(9), "9->10");

        let mut buf: Vec<&i32> = vec![];
        for item in tree.into_iter() {
            buf.push(item);
        }
        assert_eq!(buf, vec![&1, &2, &3, &4, &6, &7, &8, &9, &10]);
    }

    #[cfg(feature = "reverse-in-order")]
    #[test]
    fn test_dfs_iter_reverse_in_order() {
        let mut tree: RedBlackTree<i32> = RedBlackTree::new();
        let nodes: Vec<i32> = vec![4, 3, 9, 1, 2, 10, 7, 8, 6];

        for value in nodes {
            tree.insert(value);
        }

        let values: Vec<&i32> = tree.iter_reverse_inorder().collect();

        assert_eq!(values, vec![&10, &9, &8, &7, &6, &4, &3, &2, &1]);

        assert_eq!(Some(&9), tree.successor_dfs_reverse_in_order(10));
        assert_eq!(Some(&8), tree.successor_dfs_reverse_in_order(9));
        assert_eq!(Some(&7), tree.successor_dfs_reverse_in_order(8));
        assert_eq!(Some(&6), tree.successor_dfs_reverse_in_order(7));
        assert_eq!(Some(&4), tree.successor_dfs_reverse_in_order(6));
        assert_eq!(Some(&3), tree.successor_dfs_reverse_in_order(4));
        assert_eq!(Some(&2), tree.successor_dfs_reverse_in_order(3));
        assert_eq!(Some(&1), tree.successor_dfs_reverse_in_order(2));
        assert_eq!(None, tree.successor_dfs_reverse_in_order(1));
    }

    // $ cargo test test_dfs_iter_pre_order --no-default-features --features pre-order -- --nocapture
    #[cfg(feature = "pre-order")]
    #[test]
    fn test_dfs_iter_pre_order() {
        let mut tree: RedBlackTree<i32> = RedBlackTree::new();
        let nodes: Vec<i32> = vec![4, 3, 9, 1, 2, 10, 7, 8, 6];
        for i in nodes {
            tree.insert(i);
        }

        let mut buf: Vec<&i32> = vec![];
        for item in tree.iter_preorder() {
            buf.push(item);
        }
        assert_eq!(buf, vec![&4, &2, &1, &3, &9, &7, &6, &8, &10]);

        assert_eq!(Some(&2), tree.successor_dfs_pre_order(4), "4->2");
        assert_eq!(Some(&1), tree.successor_dfs_pre_order(2), "2->1");
        assert_eq!(Some(&3), tree.successor_dfs_pre_order(1), "1->3");
        assert_eq!(Some(&9), tree.successor_dfs_pre_order(3), "3->9");
        assert_eq!(Some(&7), tree.successor_dfs_pre_order(9), "9->7");
        assert_eq!(Some(&6), tree.successor_dfs_pre_order(7), "7->6");
        assert_eq!(Some(&8), tree.successor_dfs_pre_order(6), "6->8");
        assert_eq!(Some(&10), tree.successor_dfs_pre_order(8), "8->10");

        // Default IntoIterator = in-order
        let mut buf: Vec<&i32> = vec![];
        for item in tree.into_iter() {
            buf.push(item);
        }
        assert_eq!(buf, vec![&1, &2, &3, &4, &6, &7, &8, &9, &10]);

        // For pre-order
        let mut buf: Vec<&i32> = vec![];
        for item in tree.iter_preorder() {
            buf.push(item);
        }
        assert_eq!(buf, vec![&4, &2, &1, &3, &9, &7, &6, &8, &10]);
    }

    // $ cargo test test_dfs_iter_post_order --no-default-features --features post-order -- --nocapture
    #[cfg(feature = "post-order")]
    #[test]
    fn test_dfs_iter_post_order() {
        let mut tree: RedBlackTree<i32> = RedBlackTree::new();
        let nodes: Vec<i32> = vec![4, 3, 9, 1, 2, 10, 7, 8, 6];
        for i in nodes {
            tree.insert(i);
        }

        let mut buf: Vec<&i32> = vec![];
        for item in tree.iter_postorder() {
            buf.push(item);
        }
        assert_eq!(buf, vec![&1, &3, &2, &6, &8, &7, &10, &9, &4]);

        assert_eq!(Some(&3), tree.successor_dfs_post_order(1), "1->3");
        assert_eq!(Some(&2), tree.successor_dfs_post_order(3), "3->2");
        assert_eq!(Some(&6), tree.successor_dfs_post_order(2), "2->6");
        assert_eq!(Some(&8), tree.successor_dfs_post_order(6), "6->8");
        assert_eq!(Some(&7), tree.successor_dfs_post_order(8), "8->7");
        assert_eq!(Some(&10), tree.successor_dfs_post_order(7), "7->10");
        assert_eq!(Some(&9), tree.successor_dfs_post_order(10), "10->9");
        assert_eq!(Some(&4), tree.successor_dfs_post_order(9), "9->4");

        // Default IntoIterator = in-order
        let mut buf: Vec<&i32> = vec![];
        for item in tree.into_iter() {
            buf.push(item);
        }
        assert_eq!(buf, vec![&1, &2, &3, &4, &6, &7, &8, &9, &10]);

        // For post-order
        let mut buf: Vec<&i32> = vec![];
        for item in tree.iter_postorder() {
            buf.push(item);
        }
        assert_eq!(buf, vec![&1, &3, &2, &6, &8, &7, &10, &9, &4]);
    }

    // $ cargo test test_bfs -- --nocapture
    #[cfg(feature = "bfs")]
    #[test]
    fn test_bfs() {
        let mut tree: RedBlackTree<i32> = RedBlackTree::new();
        let nodes: Vec<i32> = vec![4, 3, 9, 1, 2, 10, 7, 8, 6];
        for i in nodes {
            tree.insert(i);
        }

        let buf: Vec<&i32> = tree.iter_level_order();
        assert_eq!(buf, vec![&4, &2, &9, &1, &3, &7, &10, &6, &8]);
    }

    #[test]
    fn test_basic_tree_api() {
        let mut tree = RedBlackTree::<i32>::new();

        assert_eq!(tree.len(), 0);
        assert!(tree.is_empty());
        assert_eq!(tree.last(), None);
        assert_eq!(tree.last(), None);

        for value in [8, 3, 10, 1, 6, 14, 4, 7, 13] {
            tree.insert(value);
        }

        assert_eq!(tree.first(), Some(&1));
        assert_eq!(tree.last(), Some(&14));

        tree.clear();

        assert_eq!(tree.first(), None);
        assert_eq!(tree.last(), None);

        assert!(tree.insert(20));

        assert_eq!(tree.first(), Some(&20));
        assert_eq!(tree.last(), Some(&20));
    }

    #[test]
    fn test_height() {
        let mut tree = RedBlackTree::<i32>::new();

        assert_eq!(tree.height(), 0);

        tree.insert(10);
        assert_eq!(tree.height(), 1);

        tree.insert(5);
        assert_eq!(tree.height(), 2);

        tree.insert(15);
        assert_eq!(tree.height(), 2);

        tree.insert(1);

        // After balancing, the exact shape of the tree may differ,
        // but its height should remain 3 or less for this set of values.
        assert!(tree.height() <= 3);

        assert!(tree.is_valid());
    }

    #[test]
    fn test_height_balanced() {
        let mut tree = RedBlackTree::<i32>::new();

        for value in 0..1000 {
            tree.insert(value);
        }

        let height = tree.height();

        assert!(tree.is_valid());

        // A regular, degenerate BST would have a height of 1000 here.
        // A Red-Black Tree should have a height of O(log n).
        assert!(height < 30, "height = {}", height);
    }

    #[test]
    fn test_successor_predecessor() {
        let mut tree = RedBlackTree::<i32>::new();

        for value in [8, 3, 10, 1, 6, 14, 4, 7, 13] {
            tree.insert(value);
        }

        // successor
        assert_eq!(tree.successor(&1), Some(&3));
        assert_eq!(tree.successor(&3), Some(&4));
        assert_eq!(tree.successor(&4), Some(&6));
        assert_eq!(tree.successor(&6), Some(&7));
        assert_eq!(tree.successor(&7), Some(&8));
        assert_eq!(tree.successor(&8), Some(&10));
        assert_eq!(tree.successor(&10), Some(&13));
        assert_eq!(tree.successor(&13), Some(&14));
        assert_eq!(tree.successor(&14), None);

        // predecessor
        assert_eq!(tree.predecessor(&14), Some(&13));
        assert_eq!(tree.predecessor(&13), Some(&10));
        assert_eq!(tree.predecessor(&10), Some(&8));
        assert_eq!(tree.predecessor(&8), Some(&7));
        assert_eq!(tree.predecessor(&7), Some(&6));
        assert_eq!(tree.predecessor(&6), Some(&4));
        assert_eq!(tree.predecessor(&4), Some(&3));
        assert_eq!(tree.predecessor(&3), Some(&1));
        assert_eq!(tree.predecessor(&1), None);

        assert_eq!(tree.successor(&5), None);
        assert_eq!(tree.predecessor(&5), None);
    }

    #[test]
    fn test_lower_upper_bound() {
        let mut tree = RedBlackTree::<i32>::new();

        for value in [8, 3, 10, 1, 6, 14, 4, 7, 13] {
            tree.insert(value);
        }

        // lower_bound: первый >= value
        assert_eq!(tree.lower_bound(&0), Some(&1));
        assert_eq!(tree.lower_bound(&1), Some(&1));
        assert_eq!(tree.lower_bound(&5), Some(&6));
        assert_eq!(tree.lower_bound(&6), Some(&6));
        assert_eq!(tree.lower_bound(&11), Some(&13));
        assert_eq!(tree.lower_bound(&14), Some(&14));
        assert_eq!(tree.lower_bound(&15), None);

        // upper_bound: первый > value
        assert_eq!(tree.upper_bound(&0), Some(&1));
        assert_eq!(tree.upper_bound(&1), Some(&3));
        assert_eq!(tree.upper_bound(&5), Some(&6));
        assert_eq!(tree.upper_bound(&6), Some(&7));
        assert_eq!(tree.upper_bound(&10), Some(&13));
        assert_eq!(tree.upper_bound(&13), Some(&14));
        assert_eq!(tree.upper_bound(&14), None);
    }

    #[test]
    fn test_bounds_empty_tree() {
        let tree = RedBlackTree::<i32>::new();

        assert_eq!(tree.lower_bound(&10), None);
        assert_eq!(tree.upper_bound(&10), None);
    }

    #[test]
    fn test_range() {
        let mut tree = RedBlackTree::<i32>::new();

        for value in [8, 3, 10, 1, 6, 14, 4, 7, 13] {
            tree.insert(value);
        }

        assert_eq!(
            tree.range(&4, &13).copied().collect::<Vec<_>>(),
            vec![4, 6, 7, 8, 10, 13]
        );

        assert_eq!(
            tree.range(&1, &14).copied().collect::<Vec<_>>(),
            vec![1, 3, 4, 6, 7, 8, 10, 13, 14]
        );

        assert_eq!(
            tree.range(&5, &9).copied().collect::<Vec<_>>(),
            vec![6, 7, 8]
        );

        assert_eq!(
            tree.range(&20, &30).copied().collect::<Vec<_>>(),
            Vec::<i32>::new()
        );

        // min > max
        assert_eq!(
            tree.range(&10, &5).copied().collect::<Vec<_>>(),
            Vec::<i32>::new()
        );
    }

    #[test]
    fn test_range_empty_tree() {
        let tree = RedBlackTree::<i32>::new();

        assert_eq!(tree.range(&1, &10).next(), None);
    }

    #[test]
    fn test_bfs_empty_tree() {
        let tree = RedBlackTree::<i32>::new();

        assert!(tree.iter_level_order().is_empty());
    }

    #[test]
    fn test_stress_insert() {
        let mut tree = RedBlackTree::<i32>::new();

        for value in 0..1000 {
            assert!(tree.insert(value));
            assert!(tree.is_valid());
        }

        assert_eq!(tree.len(), 1000);

        let values: Vec<i32> = tree.iter().copied().collect();

        assert_eq!(values, (0..1000).collect::<Vec<_>>());
        assert!(tree.height() < 30);
    }

    #[test]
    fn test_stress_remove() {
        let mut tree = RedBlackTree::<i32>::new();

        for value in 0..500 {
            tree.insert(value);
        }

        assert!(tree.is_valid());

        for value in 0..500 {
            assert!(tree.remove(&value));
            assert!(
                tree.is_valid(),
                "tree became invalid after removing {}",
                value
            );
        }

        assert_eq!(tree.len(), 0);
        assert!(tree.is_empty());
        assert_eq!(tree.height(), 0);
    }

    #[test]
    fn test_stress_remove_mixed_order() {
        let mut tree = RedBlackTree::<i32>::new();

        let values = [50, 25, 75, 10, 30, 60, 90, 5, 15, 27, 35, 55, 65, 80, 95];

        for value in values {
            assert!(tree.insert(value));
        }

        assert!(tree.is_valid());

        for value in [30, 75, 5, 90, 25, 65, 10, 95, 50, 15, 27, 35, 55, 60, 80] {
            assert!(tree.remove(&value));
            assert!(tree.is_valid(), "invalid after remove({})", value);
        }

        assert!(tree.is_empty());
    }

    #[test]
    fn test_take() {
        let mut tree = RedBlackTree::new();

        for value in [8, 3, 10, 1, 6, 14, 4, 7, 13] {
            tree.insert(value);
        }

        assert_eq!(tree.len(), 9);

        assert_eq!(tree.take(&6), Some(6));

        assert!(!tree.contains(&6));
        assert_eq!(tree.len(), 8);
        assert!(tree.is_valid());

        assert_eq!(tree.take(&100), None);

        assert_eq!(tree.len(), 8);
        assert!(tree.is_valid());
    }

    #[test]
    fn test_take_node_with_children() {
        let mut tree = RedBlackTree::new();

        for value in [8, 3, 10, 1, 6, 14, 4, 7, 13] {
            tree.insert(value);
        }

        let removed = tree.take(&8);

        assert_eq!(removed, Some(8));
        assert!(!tree.contains(&8));

        assert_eq!(
            tree.iter().copied().collect::<Vec<_>>(),
            vec![1, 3, 4, 6, 7, 10, 13, 14]
        );

        assert!(tree.is_valid());
    }

    #[test]
    fn test_take_only_node() {
        let mut tree = RedBlackTree::new();

        tree.insert(10);

        assert_eq!(tree.take(&10), Some(10));

        assert!(tree.is_empty());
        assert_eq!(tree.len(), 0);
        assert!(tree.is_valid());
    }

    #[test]
    fn test_take_empty_tree() {
        let mut tree = RedBlackTree::<i32>::new();

        assert_eq!(tree.take(&10), None);
        assert!(tree.is_empty());
    }

    #[test]
    fn test_take_custom_type() {
        use std::cmp::Ordering;

        #[derive(Debug, Default)]
        struct User {
            id: u32,
            name: String,
        }

        impl PartialEq for User {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id
            }
        }

        impl Eq for User {}

        impl PartialOrd for User {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for User {
            fn cmp(&self, other: &Self) -> Ordering {
                self.id.cmp(&other.id)
            }
        }

        let mut tree = RedBlackTree::new();

        tree.insert(User {
            id: 1,
            name: "Alice".into(),
        });

        tree.insert(User {
            id: 2,
            name: "Charlie".into(),
        });

        tree.insert(User {
            id: 3,
            name: "Bob".into(),
        });

        // Search only by id.
        let key = User {
            id: 2,
            name: String::new(),
        };

        let removed = tree.take(&key).unwrap();

        assert_eq!(removed.id, 2);
        assert_eq!(removed.name, "Charlie");

        assert!(!tree.contains(&key));
        assert_eq!(tree.len(), 2);
        assert!(tree.is_valid());
    }

    #[test]
    fn test_replace() {
        let mut tree = RedBlackTree::new();

        tree.extend([10, 5, 20]);

        assert_eq!(tree.replace(10), Some(10));

        assert_eq!(tree.len(), 3);
        assert!(tree.contains(&10));
        assert!(tree.is_valid());

        assert_eq!(tree.replace(15), None);

        assert_eq!(tree.len(), 4);
        assert!(tree.contains(&15));
        assert!(tree.is_valid());
    }

    #[test]
    fn test_replace_custom_type() {
        use std::cmp::Ordering;

        #[derive(Debug, Default)]
        struct User {
            id: u32,
            name: String,
        }

        impl PartialEq for User {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id
            }
        }

        impl Eq for User {}

        impl PartialOrd for User {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for User {
            fn cmp(&self, other: &Self) -> Ordering {
                self.id.cmp(&other.id)
            }
        }

        let mut tree = RedBlackTree::new();

        tree.insert(User {
            id: 2,
            name: "Alice".into(),
        });

        let old = tree.replace(User {
            id: 2,
            name: "Bob".into(),
        });

        let old = old.unwrap();

        assert_eq!(old.id, 2);
        assert_eq!(old.name, "Alice");

        let key = User {
            id: 2,
            name: String::new(),
        };

        let current = tree.get(&key).unwrap();

        assert_eq!(current.id, 2);
        assert_eq!(current.name, "Bob");

        // replace does not change the number of elements
        assert_eq!(tree.len(), 1);
        assert!(tree.is_valid());
    }

    #[test]
    fn test_replace_inserts_when_missing() {
        let mut tree = RedBlackTree::new();

        assert_eq!(tree.replace(10), None);

        assert_eq!(tree.len(), 1);
        assert!(tree.contains(&10));
        assert!(tree.is_valid());
    }

    #[test]
    fn test_pop_first() {
        let mut tree = RedBlackTree::new();

        tree.extend([8, 3, 10, 1, 6, 14, 4, 7, 13]);

        let mut values = Vec::new();

        while let Some(value) = tree.pop_first() {
            values.push(value);
            assert!(tree.is_valid());
        }

        assert_eq!(values, vec![1, 3, 4, 6, 7, 8, 10, 13, 14]);

        assert!(tree.is_empty());
        assert_eq!(tree.pop_first(), None);
    }

    #[test]
    fn test_pop_last() {
        let mut tree = RedBlackTree::new();

        tree.extend([8, 3, 10, 1, 6, 14, 4, 7, 13]);

        let mut values = Vec::new();

        while let Some(value) = tree.pop_last() {
            values.push(value);
            assert!(tree.is_valid());
        }

        assert_eq!(values, vec![14, 13, 10, 8, 7, 6, 4, 3, 1]);

        assert!(tree.is_empty());
        assert_eq!(tree.pop_last(), None);
    }

    #[test]
    fn test_pop_empty_tree() {
        let mut tree = RedBlackTree::<i32>::new();

        assert_eq!(tree.pop_first(), None);
        assert_eq!(tree.pop_last(), None);
    }

    #[test]
    fn test_retain() {
        let mut tree = RedBlackTree::new();

        tree.extend([8, 3, 10, 1, 6, 14, 4, 7, 13]);

        tree.retain(|value| *value >= 7);

        assert_eq!(
            tree.iter().copied().collect::<Vec<_>>(),
            vec![7, 8, 10, 13, 14]
        );

        assert_eq!(tree.len(), 5);
        assert!(tree.is_valid());
    }

    #[test]
    fn test_retain_none() {
        let mut tree = RedBlackTree::new();

        tree.extend([1, 2, 3, 4, 5]);

        tree.retain(|_| false);

        assert!(tree.is_empty());
        assert_eq!(tree.len(), 0);
        assert!(tree.is_valid());
    }

    #[test]
    fn test_retain_all() {
        let mut tree = RedBlackTree::new();

        tree.extend([5, 1, 10, 3]);

        tree.retain(|_| true);

        assert_eq!(tree.iter().copied().collect::<Vec<_>>(), vec![1, 3, 5, 10]);

        assert_eq!(tree.len(), 4);
        assert!(tree.is_valid());
    }

    #[test]
    fn test_retain_custom_type() {
        use std::cmp::Ordering;

        #[derive(Debug, Default)]
        struct Task {
            id: u32,
            priority: u32,
        }

        impl PartialEq for Task {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id
            }
        }

        impl Eq for Task {}

        impl PartialOrd for Task {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for Task {
            fn cmp(&self, other: &Self) -> Ordering {
                self.id.cmp(&other.id)
            }
        }

        let mut tasks = RedBlackTree::new();

        tasks.insert(Task {
            id: 1,
            priority: 10,
        });

        tasks.insert(Task {
            id: 2,
            priority: 100,
        });

        tasks.insert(Task {
            id: 3,
            priority: 50,
        });

        tasks.retain(|task| task.priority >= 50);

        assert_eq!(tasks.len(), 2);

        let ids: Vec<u32> = tasks.iter().map(|task| task.id).collect();

        assert_eq!(ids, vec![2, 3]);
        assert!(tasks.is_valid());
    }

    #[test]
    fn test_iter_exact_size() {
        let mut tree = RedBlackTree::new();

        tree.extend([1, 2, 3, 4, 5]);

        let mut iter = tree.iter();

        assert_eq!(iter.len(), 5);
        assert_eq!(iter.size_hint(), (5, Some(5)));

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.len(), 4);

        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.len(), 3);
    }

    #[test]
    fn test_iter_fused() {
        let mut tree = RedBlackTree::new();

        tree.extend([1, 2]);

        let mut iter = tree.iter();

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));

        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_range_iterator() {
        let mut tree = RedBlackTree::new();

        tree.extend([8, 3, 10, 1, 6, 14, 4, 7, 13]);

        let values: Vec<i32> = tree.range(&4, &10).copied().collect();

        assert_eq!(values, vec![4, 6, 7, 8, 10]);
    }

    #[test]
    fn test_range_iterator_missing_bounds() {
        let mut tree = RedBlackTree::new();

        tree.extend([10, 20, 30, 40, 50]);

        let values: Vec<i32> = tree.range(&15, &45).copied().collect();

        assert_eq!(values, vec![20, 30, 40]);
    }

    #[test]
    fn test_range_iterator_empty() {
        let mut tree = RedBlackTree::new();

        tree.extend([10, 20, 30]);

        assert_eq!(tree.range(&40, &50).next(), None);
    }

    #[test]
    fn test_range_iterator_invalid_bounds() {
        let mut tree = RedBlackTree::new();

        tree.extend([1, 2, 3, 4, 5]);

        assert_eq!(tree.range(&4, &2).next(), None);
    }

    #[test]
    fn test_range_take() {
        let mut tree = RedBlackTree::new();

        tree.extend(1..=100);

        assert_eq!(
            tree.range(&20, &80).take(3).copied().collect::<Vec<_>>(),
            vec![20, 21, 22]
        );
    }
}
