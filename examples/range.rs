use rbtree_extended::RedBlackTree;

fn main() {
    let mut tree = RedBlackTree::new();

    for value in [1, 3, 4, 6, 7, 8, 10, 13, 14] {
        tree.insert(value);
    }

    print!("tree: ");
    for value in tree.iter() {
        print!("{value} ");
    }
    println!();

    // lower_bound:
    // first element >= value
    println!("lower_bound(5) = {:?}", tree.lower_bound(&5));
    println!("lower_bound(6) = {:?}", tree.lower_bound(&6));
    println!("lower_bound(15) = {:?}", tree.lower_bound(&15));

    // upper_bound:
    // first element > value
    println!("upper_bound(5) = {:?}", tree.upper_bound(&5));
    println!("upper_bound(6) = {:?}", tree.upper_bound(&6));
    println!("upper_bound(14) = {:?}", tree.upper_bound(&14));

    // Inclusive ranges
    println!(
        "range 4..=10 = {:?}",
        tree.range(&4, &10).copied().collect::<Vec<_>>()
    );

    println!(
        "range 5..=9  = {:?}",
        tree.range(&5, &9).copied().collect::<Vec<_>>()
    );

    println!(
        "range 1..=14 = {:?}",
        tree.range(&1, &14).copied().collect::<Vec<_>>()
    );

    // Range without matching values
    println!(
        "range 20..=30 = {:?}",
        tree.range(&20, &30).copied().collect::<Vec<_>>()
    );

    // Invalid range: min > max
    println!(
        "range 10..=5 = {:?}",
        tree.range(&10, &5).copied().collect::<Vec<_>>()
    );

    // Ordered navigation
    println!("first = {:?}", tree.first());
    println!("last = {:?}", tree.last());

    println!("successor(8) = {:?}", tree.successor(&8));
    println!("predecessor(8) = {:?}", tree.predecessor(&8));

    println!("successor(14) = {:?}", tree.successor(&14));
    println!("predecessor(1) = {:?}", tree.predecessor(&1));
}
