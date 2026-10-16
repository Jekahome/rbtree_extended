//! Using `RedBlackTree` as a priority queue.
//!
//! Because the tree always keeps its values ordered according to [`Ord`],
//! the largest element can be removed efficiently with
//! [`RedBlackTree::pop_last`].
//!
//! Here, tasks are ordered first by `priority` and then by `id`. The `id` acts
//! as a tie-breaker so that multiple tasks with the same priority can coexist
//! in the tree.
//!
//! Repeatedly calling `pop_last()` processes the highest-priority task first.

use std::cmp::Ordering;

use rbtree_extended::RedBlackTree;

#[derive(Debug, Default)]
struct Task {
    id: u32,
    priority: u32,
    name: String,
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority && self.id == other.id
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
        self.priority
            .cmp(&other.priority)
            .then_with(|| self.id.cmp(&other.id))
    }
}

// cargo run --example priority_queue

fn main() {
    let mut queue = RedBlackTree::new();

    queue.insert(Task {
        id: 1,
        priority: 10,
        name: "Send email".into(),
    });

    queue.insert(Task {
        id: 2,
        priority: 100,
        name: "Fix production".into(),
    });

    queue.insert(Task {
        id: 3,
        priority: 50,
        name: "Review pull request".into(),
    });

    queue.insert(Task {
        id: 4,
        priority: 100,
        name: "Restore database backup".into(),
    });

    println!("Processing highest-priority tasks first:");

    while let Some(task) = queue.pop_last() {
        println!(
            "priority={} id={} task={}",
            task.priority, task.id, task.name
        );
    }
}
