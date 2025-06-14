use std::time::Duration;
use tokio::time::sleep;

async fn async_fn() -> i32 {
    println!("This is inside async fn 1");
    println!("This is inside async fn 2");
    println!("Waiting for some time...");
    sleep(Duration::from_secs(2)).await;
    println!("Wait completed");
    println!("Exiting async fn");
    10
}

#[tokio::main]
async fn main() {
    println!("This is main fn");
    let no: i32 = async_fn().await;
    println!("Returned from async_fn: {}", no);
    println!("The main ending here");
}
