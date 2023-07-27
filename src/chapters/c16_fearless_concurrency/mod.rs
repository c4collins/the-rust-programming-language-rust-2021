//! Concurrency, and or parallelism, uses threads to enacapsulate processes
//! And either the threads pass messages, or have a shared state, to maintain data consistency.

use std::sync::mpsc; // MP;SC = multiple producer, single consumer Type
use std::sync::Arc; // ARC = atomically reference counted Type
use std::sync::Mutex; // Mut-Ex = mutual exclusion Type
use std::thread;
use std::time::Duration;

/// This function will run all of the code I wrote for Chapter 16
pub fn run() {
    thread_example();
    thread_closure_ownership();
    thread_channels();
    handling_multiple_messages();
    multiple_transmitters();
    simple_mutex();
    thread_shared_mutex();
}

// Example showing concurrency leads to unpredictable execution order
fn thread_example() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // handle.join().unwrap(); // If you use this join() instead of the one below
    // all of this first loop will complete before the second one (negating the benefit of threading)

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // println!("Anything past here would not happen if this was the main file/process in a program"); // Was true before join()
    handle.join().unwrap();
}

// Example showing how to move ownership into a thread
fn thread_closure_ownership() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

// Example showing how to use channels to pass messages between threads
fn thread_channels() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val1 = String::from("hi");
        tx.send(val1).unwrap();

        thread::sleep(Duration::from_millis(1));

        let val2 = String::from("hello");
        tx.send(val2).unwrap();
    });

    {
        // Simple rx.recv() usage example
        let received = rx.recv().unwrap();
        println!("Got: {}", received);
    }

    // thread::sleep(Duration::from_millis(10)); // This can be used to show that "Waiting..." below only shows in some situations

    // Using try_recv() instead of recv()
    loop {
        match rx.try_recv() {
            Ok(msg) => {
                println!("Got: {}", msg);
                break;
            }
            Err(_) => {
                println!("Waiting...");
                thread::sleep(Duration::from_millis(1)); // Commenting this out will make this happen 5-100 times before the message is received.
            }
        }
    }
}

// Example showing how to use rx as an iterator to wait for messages
fn handling_multiple_messages() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

// Example showing how to clone tx to have multiple transmitters
fn multiple_transmitters() {
    let (t1_tx, rx) = mpsc::channel();
    let t2_tx = t1_tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            t1_tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            t2_tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

// Eample showing basic Mutex usage
fn simple_mutex() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = *num + 6;
        // lock is released automatically when num goes out of scope
    }

    println!("m = {:?}", m);
}

// Example showing using a mutex shared among threads
fn thread_shared_mutex() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Results: {}", *counter.lock().unwrap());
}
