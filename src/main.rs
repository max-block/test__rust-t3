use std::{
    thread::{sleep, spawn},
    time::Duration,
};

macro_rules! synchronized {
    () => {
        lazy_static::lazy_static! {
            static ref LOCK: std::sync::Mutex<i32> = std::sync::Mutex::new(0);
        }
        let _r = LOCK.lock().unwrap();
    };
}

fn f1(a: i32) {
    synchronized!();
    println!("f1 start / {}", a);
    sleep(Duration::from_secs(5));
    println!("f1 finish / {}", a);
}

fn f2(a: i32) {
    synchronized!();
    println!("f2 start / {}", a);
    sleep(Duration::from_secs(5));
    println!("f2 finish / {}", a);
}

fn main() {
    spawn(|| f1(1));
    spawn(|| f1(2));
    spawn(|| f2(1));
    spawn(|| f2(2));
    sleep(Duration::from_secs(1000));
}
