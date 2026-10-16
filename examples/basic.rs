use rbtree_extended::RedBlackTree;

// cargo run --example basic

fn main() {
    let mut tree = RedBlackTree::new();

    // Insert
    for value in [8, 3, 10, 1, 6, 14, 4, 7, 13] {
        tree.insert(value);
    }

    println!("len = {}", tree.len());
    println!("is_empty = {}", tree.is_empty());

    // Duplicate values are not inserted
    println!("insert duplicate 6 = {}", tree.insert(6));

    // Search
    println!("contains 7 = {}", tree.contains(&7));
    println!("contains 12 = {}", tree.contains(&12));

    println!("get 6 = {:?}", tree.get(&6));
    println!("get 12 = {:?}", tree.get(&12));

    // Minimum / maximum
    println!("first = {:?}", tree.first());
    println!("last = {:?}", tree.last());

    // Successor / predecessor
    println!("successor(7) = {:?}", tree.successor(&7));
    println!("predecessor(7) = {:?}", tree.predecessor(&7));

    // Height
    println!("height = {}", tree.height());

    // Default sorted iteration
    print!("sorted: ");
    for value in tree.iter() {
        print!("{value} ");
    }
    println!();

    // Removal
    println!("remove 6 = {}", tree.remove(&6));
    println!("remove 100 = {}", tree.remove(&100));

    print!("after remove: ");
    for value in tree.iter() {
        print!("{value} ");
    }
    println!();

    // Structural validation
    println!("valid = {}", tree.is_valid());

    // FromIterator
    let collected: RedBlackTree<i32> = [50, 20, 70, 10, 30, 60, 80].into_iter().collect();

    print!("collected: ");
    for value in collected.iter() {
        print!("{value} ");
    }
    println!();

    // Extend
    let mut extended = RedBlackTree::new();
    extended.extend([5, 1, 9, 3, 7]);

    print!("extended: ");
    for value in extended.iter() {
        print!("{value} ");
    }
    println!();

    // Clone
    let cloned = extended.clone();

    println!("tree == clone: {}", extended == cloned);

    // Clear
    tree.clear();

    println!("after clear:");
    println!("len = {}", tree.len());
    println!("is_empty = {}", tree.is_empty());
}
