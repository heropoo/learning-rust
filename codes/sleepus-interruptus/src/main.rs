use std::{
    thread::{sleep, spawn},
    time::Duration,
};

fn sleepus() {
    for i in 1..10 {
        println!("Sleep {}", i);
        sleep(Duration::from_millis(500));
    }
}

fn interruptus() {
    for i in 1..5 {
        println!("Interruptus {}", i);
        sleep(Duration::from_millis(1000));
    }
}

// see https://blog.csdn.net/shebao3333/article/details/106851398/

fn main() {
    //sleepus();
    //interruptus();

    // 1. 为每个函数创建一个单独的线程
    // let sleepus = spawn(sleepus);
    // let interruptus = spawn(interruptus);
    // sleepus.join().unwrap();
    // interruptus.join().unwrap();

    // 2. 创建一个辅助线程，然后在主线程种调用其中一个函数
    let sleepus = spawn(sleepus);
    interruptus();
    sleepus.join().unwrap();
}
