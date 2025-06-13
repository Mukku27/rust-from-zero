use std::thread;

fn main() {
    // A vector owned by the main thread
    let v = vec![1, 2, 3];

    // `move` transfers ownership of `v` into the closure
    let handle = thread::spawn(move || {
        println!("✅ Thread has taken ownership of the vector: {:?}", v);

        // Simulate some processing
        let squared: Vec<i32> = v.iter().map(|x| x * x).collect();
        println!("🧮 Squared values inside thread: {:?}", squared);
    });

    // Ensure main waits for the spawned thread
    match handle.join() {
        Ok(_) => println!("✔️ Thread completed successfully."),
        Err(_) => println!("❌ Thread panicked."),
    }

    // ❌ Uncommenting the line below will cause a compile-time error
    // because ownership of `v` was moved into the thread.
    // println!("Trying to use v: {:?}", v); // ERROR: borrow of moved value: `v`
}
