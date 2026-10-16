use rbtree_extended::RedBlackTree;

// cargo run --example dot
fn main() {
    let mut tree = RedBlackTree::new();

    for value in [8, 3, 10, 1, 6, 14, 4, 7, 13] {
        tree.insert(value);
    }

    let dot = tree.to_dot();

    println!("{dot}");
    // TODO: To view the tree, paste the resulting string into dot online:
    // * https://dreampuf.github.io/GraphvizOnline/
    // * http://www.webgraphviz.com/?tab=map
}
