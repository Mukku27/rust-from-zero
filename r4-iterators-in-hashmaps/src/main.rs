use std::collections::HashMap;

fn main() {
    // Create a HashMap and populate it with some key-value pairs
    let mut scores = HashMap::new();
    scores.insert("Alice", 50);
    scores.insert("Bob", 40);
    scores.insert("Charlie", 30);

    // Example 1: Iterating over references to key-value pairs
    println!("Iterating over key-value pairs:");
    for (key, value) in scores.iter() {
        println!("{}: {}", key, value);
    }

    // Example 2: Iterating over mutable references to key-value pairs
    println!("\nIterating over mutable key-value pairs:");
    for (key, value) in scores.iter_mut() {
        *value += 10; // Increment each score by 10
        println!("{}: {}", key, value);
    }
}