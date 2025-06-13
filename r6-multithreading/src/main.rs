use std::thread;
use std::time::Duration;

fn main() {
    // Spawned Thread 1
    let handle1 = thread::spawn(|| {
        for i in 1..5 {
            println!("🧵 Thread-1 says: {i}");
            thread::sleep(Duration::from_millis(10));
        }
    });

    // Spawned Thread 2
    let handle2 = thread::spawn(|| {
        for i in 1..5 {
            println!("🧵 Thread-2 says: {i}");
            thread::sleep(Duration::from_millis(8));
        }
    });

    // Main Thread Work
    for i in 1..5 {
        println!("🧠 Main thread says: {i}");
        thread::sleep(Duration::from_millis(6));
    }

    // Wait for both spawned threads to finish
    handle1.join().unwrap();
    handle2.join().unwrap();
}
