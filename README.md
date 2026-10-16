# rbtree_extended

`rbtree_extended` is an extended Red-Black Tree implementation written in Rust.

[![CI](https://github.com/YOUR_GITHUB_USERNAME/rbtree-extended/actions/workflows/ci.yml/badge.svg)](https://github.com/YOUR_GITHUB_USERNAME/rbtree-extended/actions/workflows/ci.yml)
[![release](https://img.shields.io/github/v/release/YOUR_GITHUB_USERNAME/rbtree-extended?sort=semver)](https://github.com/YOUR_GITHUB_USERNAME/rbtree-extended/releases)
[![crates.io](https://img.shields.io/crates/v/rbtree_extended.svg)](https://crates.io/crates/rbtree_extended)
[![docs.rs](https://docs.rs/rbtree_extended/badge.svg)](https://docs.rs/rbtree_extended)
[![license](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](LICENSE-MIT)

A Red-Black Tree is a self-balancing binary search tree that keeps search, insertion, and removal operations efficient even as the tree grows.

This crate provides the usual ordered-tree operations together with traversal iterators, range queries, bounds, value replacement, filtering, and ordered extraction.

## Features

- Red-Black Tree insertion and removal
- Search with `get()` and `contains()`
- Minimum and maximum values
- `successor()` and `predecessor()`
- `lower_bound()` and `upper_bound()`
- Lazy inclusive `range()` iterator
- In-order traversal
- Reverse in-order traversal
- Pre-order traversal
- Post-order traversal
- Level-order traversal
- `take()` — remove and return a value
- `replace()` — replace an equal value or insert a new one
- `pop_first()` / `pop_last()` — remove the smallest or largest value
- `retain()` — keep only values matching a predicate
- Graphviz DOT export
- Tree validation with `is_valid()`

## Installation

Add the crate to your project:

```toml
[dependencies]
rbtree_extended = "0.1"
```

## Basic usage

```rust
use rbtree_extended::RedBlackTree;

let mut tree = RedBlackTree::new();

tree.insert(8);
tree.insert(3);
tree.insert(10);
tree.insert(1);
tree.insert(6);

assert!(tree.contains(&6));
assert_eq!(tree.first(), Some(&1));
assert_eq!(tree.last(), Some(&10));

let values: Vec<_> = tree.iter().copied().collect();

assert_eq!(values, vec![1, 3, 6, 8, 10]);
```

## Range iteration

`range()` is lazy and yields values in ascending order without creating an intermediate collection.

```rust
use rbtree_extended::RedBlackTree;

let mut tree = RedBlackTree::new();
tree.extend([1, 3, 5, 7, 9, 11]);

let values: Vec<_> = tree
    .range(&3, &9)
    .copied()
    .collect();

assert_eq!(values, vec![3, 5, 7, 9]);
```

Standard iterator adapters can be used directly:

```rust
let first_two: Vec<_> = tree
    .range(&3, &11)
    .take(2)
    .copied()
    .collect();

assert_eq!(first_two, vec![3, 5]);
```

## Ordered extraction

The smallest or largest value can be removed directly:

```rust
use rbtree_extended::RedBlackTree;

let mut tree = RedBlackTree::new();
tree.extend([8, 3, 10, 1, 6]);

assert_eq!(tree.pop_first(), Some(1));
assert_eq!(tree.pop_last(), Some(10));
```

This can be useful for processing sorted data or implementing priority-based processing.

## Custom types

Any type implementing `Ord` and `Default` can currently be stored in the tree.

```rust
use std::cmp::Ordering;
use rbtree_extended::RedBlackTree;

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

let mut users = RedBlackTree::new();

users.insert(User {
    id: 1,
    name: "Alice".into(),
});

let old = users.replace(User {
    id: 1,
    name: "Bob".into(),
});

assert_eq!(old.unwrap().name, "Alice");
```

## Traversal features

Traversal implementations are controlled through Cargo features:

- `in-order`
- `reverse-in-order`
- `pre-order`
- `post-order`
- `bfs`

All traversal features are enabled by default.

## Graphviz DOT export

A tree can be exported to the
[Graphviz DOT](https://graphviz.org/doc/info/lang.html) format.

This is useful for inspecting the real internal structure of the tree after
balancing.


```
use rbtree_extended::RedBlackTree;

let mut tree = RedBlackTree::new();

for value in [8, 3, 10, 1, 6, 14, 4, 7, 13] {
    tree.insert(value);
}

let dot = tree.to_dot();

println!("{dot}");
```

The resulting string can be rendered by Graphviz or pasted into an [online
DOT viewer](https://dreampuf.github.io/GraphvizOnline/).

![Red-Black Tree example](https://github.com/Jekahome/rbtree_extended/blob/main/docs/rbtree-example.svg)


A generated graph contains both the tree connections and the Red-Black
colors of the nodes.

An empty tree also produces a valid DOT graph.

## Complexity

Typical Red-Black Tree operations are logarithmic:

| Operation | Complexity |
| --- | --- |
| Search | `O(log n)` |
| Insert | `O(log n)` |
| Remove | `O(log n)` |
| Min / Max | `O(log n)` |
| Lower / Upper bound | `O(log n)` |
| Range iteration | `O(log n + k)` |
| Full traversal | `O(n)` |

where `k` is the number of values produced by a range query.

## Documentation

Full API documentation is available on [docs.rs](https://docs.rs/rbtree_extended).

The repository also contains runnable examples covering traversal, range queries, custom types, priority queues, sorted processing, and Graphviz output.

## Safety

The tree uses `Option<NonNull<Node<T>>>` internally to represent parent and child links.

This gives the implementation direct pointer-based tree navigation and rotation while keeping the public API safe. The unsafe implementation maintains ownership, parent-link, and tree-structure invariants internally.

## License

Licensed under the MIT License.