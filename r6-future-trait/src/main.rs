use futures::{executor::block_on, join};
use std::time::Duration;
use std::thread;

/// A basic async function that simulates a delay
async fn fetch_user() -> String {
    println!("Fetching user from database...");
    thread::sleep(Duration::from_secs(2)); // simulate blocking (for demo only)
    "User: Mukesh".to_string()
}

/// Another async function
async fn fetch_posts() -> String {
    println!("Fetching posts...");
    thread::sleep(Duration::from_secs(1)); // simulate delay
    "Posts: ['Rust', 'Async', 'Ownership']".to_string()
}

/// Entry point
fn main() {
    println!("🚀 Async Rust Demo Starts");

    // Run both async tasks concurrently using join!
    let result = block_on(async {
        let (user, posts) = join!(fetch_user(), fetch_posts());
        (user, posts)
    });

    println!("✅ Results:");
    println!("👤 {}", result.0);
    println!("📝 {}", result.1);
}
