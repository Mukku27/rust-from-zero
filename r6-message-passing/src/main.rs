use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Create a channel (tx: transmitter, rx: receiver)
    let (tx, rx) = mpsc::channel();

    // Clone the transmitter for another producer thread
    let tx1 = tx.clone();

    // First thread sends a message
    thread::spawn(move || {
        let msg = String::from("📨 Hi, I am from thread-1");
        thread::sleep(Duration::from_millis(100)); // Simulate delay
        tx.send(msg).unwrap();
    });

    // Second thread sends another message
    thread::spawn(move || {
        let msg = String::from("📨 Hi, I am from thread-2");
        thread::sleep(Duration::from_millis(50)); // Arrives earlier
        tx1.send(msg).unwrap();
    });

    // Receiver loop: collects messages from both threads
    for received in rx {
        println!("✅ Received: {}", received);
    }

    // When all senders go out of scope, the channel is closed
    println!("📭 All messages received. Channel closed.");
}
