use rbtree_extended::RedBlackTree;

// cargo run --example traversal

fn main() {
    let mut tree = RedBlackTree::new();

    for value in [4, 3, 9, 1, 2, 10, 7, 8, 6] {
        tree.insert(value);
    }

    print!("in-order: ");
    for value in tree.iter_inorder() {
        print!("{value} ");
    }
    println!();

    print!("reverse in-order: ");
    for value in tree.iter_reverse_inorder() {
        print!("{value} ");
    }
    println!();

    print!("pre-order: ");
    for value in tree.iter_preorder() {
        print!("{value} ");
    }
    println!();

    print!("post-order: ");
    for value in tree.iter_postorder() {
        print!("{value} ");
    }
    println!();

    print!("level-order: ");
    for value in tree.iter_level_order() {
        print!("{value} ");
    }
    println!();

    println!(
        "in-order successor of 4 = {:?}",
        tree.successor_dfs_in_order(4)
    );

    println!(
        "reverse in-order successor of 9 = {:?}",
        tree.successor_dfs_reverse_in_order(9)
    );

    println!(
        "pre-order successor of 4 = {:?}",
        tree.successor_dfs_pre_order(4)
    );

    println!(
        "post-order successor of 4 = {:?}",
        tree.successor_dfs_post_order(4)
    );
}
