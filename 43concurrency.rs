
use std::thread;
use std::time::Duration;

fn main() {
    // spawned thread created as a closure
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(2));
        }
    });

    // main thread
    for i in 1..15 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}


