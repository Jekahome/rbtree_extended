use std::cmp::Ordering;

use rbtree_extended::RedBlackTree;

#[derive(Debug, Clone, Default)]
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

// cargo run --example custom_type

fn main() {
    let mut tree = RedBlackTree::new();

    tree.insert(User {
        id: 3,
        name: "Alice".into(),
    });

    tree.insert(User {
        id: 1,
        name: "Bob".into(),
    });

    tree.insert(User {
        id: 2,
        name: "Charlie".into(),
    });

    println!("Users sorted by id:");

    for user in tree.iter() {
        println!("id={} name={}", user.id, user.name);
    }

    let key = User {
        id: 2,
        name: String::new(),
    };

    println!("contains id=2: {}", tree.contains(&key));

    if let Some(user) = tree.get(&key) {
        println!("found: {:?}", user);
    }

    println!("first = {:?}", tree.first());
    println!("last = {:?}", tree.last());

    tree.remove(&key);

    println!("after remove:");

    for user in tree.iter() {
        println!("id={} name={}", user.id, user.name);
    }

    println!("valid = {}", tree.is_valid());
}
