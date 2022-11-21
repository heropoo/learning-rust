use std::time::Duration;

use async_std::task::{sleep, spawn};

async fn sleepus() {
    for i in 1..10 {
        println!("Sleep {}", i);
        sleep(Duration::from_millis(500)).await;
    }
}

async fn interruptus() {
    for i in 1..5 {
        println!("Interruptus {}", i);
        sleep(Duration::from_millis(1000)).await;
    }
}

// see https://blog.csdn.net/shebao3333/article/details/106851398/

#[async_std::main]
async fn main() {
    let sleepus = spawn(sleepus());
    interruptus().await;

    sleepus.await;
}
