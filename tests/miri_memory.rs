use rbtree_extended::RedBlackTree;

// cargo +nightly miri test --test miri_memory
// MIRIFLAGS="-Zmiri-many-seeds=0..16" cargo +nightly miri test --test miri_memory

#[test]
fn miri_insert_remove_drop() {
    let mut tree = RedBlackTree::new();

    for value in 0..100 {
        assert!(tree.insert(value));
    }

    for value in (0..100).step_by(2) {
        assert!(tree.remove(&value));
    }

    assert!(tree.is_valid());
}

#[test]
fn miri_drop_non_empty_tree() {
    let mut tree = RedBlackTree::new();

    for value in 0..100 {
        tree.insert(value);
    }

    assert!(tree.is_valid());

    // Drop non-empty tree
}

#[test]
fn miri_clear_then_drop() {
    let mut tree = RedBlackTree::new();

    for value in 0..100 {
        tree.insert(value);
    }

    tree.clear();

    assert!(tree.is_empty());
    assert!(tree.is_valid());

    // Drop after clear
}

#[test]
fn miri_insert_remove_all() {
    let mut tree = RedBlackTree::new();

    for value in 0..100 {
        tree.insert(value);
    }

    for value in 0..100 {
        assert!(tree.remove(&value));
        assert!(tree.is_valid());
    }

    assert!(tree.is_empty());
}

#[test]
fn miri_reuse_after_clear() {
    let mut tree = RedBlackTree::new();

    for value in 0..50 {
        tree.insert(value);
    }

    tree.clear();

    for value in 100..150 {
        tree.insert(value);
    }

    assert!(tree.is_valid());
}
