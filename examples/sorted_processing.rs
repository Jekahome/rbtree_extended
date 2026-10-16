//! Processing values in sorted order.
//!
//! `RedBlackTree` keeps its elements ordered automatically, so it can be used
//! when values need to be consumed from smallest to largest without sorting
//! the entire collection separately.
//!
//! In this example, [`RedBlackTree::pop_first`] repeatedly removes the smallest
//! remaining value. This makes the tree useful as an ordered work queue or for
//! incremental processing of sorted data.

use rbtree_extended::RedBlackTree;

// cargo run --example sorted_processing

fn main() {
    let mut values = RedBlackTree::new();

    values.extend([42, 7, 19, 3, 25, 11]);

    println!("Processing values in ascending order:");

    while let Some(value) = values.pop_first() {
        println!("{value}");
    }
}
