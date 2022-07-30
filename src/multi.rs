use std::thread;
use std::time::Duration;

pub fn main() {
    // let handle = thread::spawn(|| {
    //     for i in 1..20 {
    //         println!("hello {}", i);
    //         thread::sleep(Duration::from_millis(10));
    //     }
    // });
    // handle.join().unwrap();
    // for i in 1..5 {
    //     println!("hi number {} from the main thread!", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    // Error due to rust can infer type and borrow it , but can't tell
    // if the thread run long and the reference to be valid
    // let v = vec![1, 2, 3];
    // let handle = thread::spawn(||  {
    //     println!("Here's a vector: {:?}", v);
    // });
    //
    // handle.join().unwrap();


    //move keyword for force closure to take ownership
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();
}

use std::sync::{mpsc, Arc, Mutex};

pub fn message() {
    // multiple producer single consumer
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
}


pub fn mutex() {
    // let m = Mutex::new(5);
    // {
    //     let mut num = m.lock().unwrap();
    //     *num += 1;
    // }
    // println!("m = {:?}", m);
    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}