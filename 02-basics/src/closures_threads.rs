// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_imports, unused_variables)]

// let add = |x, y| x + y;
// add(1, 2);
// We can use closures without saying the arguments we want to pass in. as it always expect a reference.
// let s = String::from("hello");
// let f = || println!("{}", s);
// f(); // prints hello

// By default we cannot pass it to another thread, we need to use `move` for that.
// let f = move || {...}

// As we know about ownership, now the s will belong to the closure, we now pass this closure to another thread or do wherever we want with it.

// Some examples of closures that I still need to understand 😅.
// let mut v = vec![2, 4, 6];
// v.iter().map(|x| x * 3).filter(|x| *x > 10).fold(0, |acc, x| acc + x);

// As many other programming languages, thread is a heavy thing to use, overall it is good in case we need to use cup and gpu in parallel but if we just need to wait something finish to continue, we can use async/await.

use crossbeam::channel;
use std::thread;
use std::time::Duration;

fn expensive_sum(v: Vec<i32>) -> i32 {
    pause_ms(500);
    println!("Child thread: just about finished");
    v.iter().filter(|&x| x % 2 == 0).map(|x| x * x).sum()
}

fn pause_ms(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}

pub fn learning_ct() {
    let my_vector = vec![2, 5, 1, 0, 4, 3];

    let handle = thread::spawn(move || expensive_sum(my_vector));

    // While the child thread is running, the main thread will also do some work
    for letter in vec!["a", "b", "c", "d", "e", "f"] {
        println!("Main thread: Letter {}", letter);
        pause_ms(200);
    }

    let sum = handle.join().unwrap();
    println!("The child thread's expensive sum is {}", sum);

    let (tx, rx) = channel::unbounded();
    // Cloning a channel makes another variable connected to that end of the channel so that you can
    // send it to another thread.
    let tx2 = tx.clone();
    let handle_a = thread::spawn(move || {
        pause_ms(200);
        tx2.send("Thread A: 1").unwrap();
        pause_ms(200);
        tx2.send("Thread A: 2").unwrap();
    });
    pause_ms(100); // Make sure Thread A has time to get going before we spawn Thread B
    let handle_b = thread::spawn(move || {
        pause_ms(0);
        tx.send("Thread B: 1").unwrap();
        pause_ms(100);
        tx.send("Thread B: 2").unwrap();
    });
    // Using a Receiver channel as an iterator is a convenient way to get values until the channel
    // gets closed.  A Receiver channel is automatically closed once all Sender channels have been
    // closed.  Both our threads automatically close their Sender channels when they exit and the
    // destructors for the channels get automatically called.
    for msg in rx {
        println!("Main thread: Received {}", msg);
    }
    // Join the child threads for good hygiene.
    handle_b.join().unwrap();
    handle_a.join().unwrap();

    let (tx, rx) = channel::unbounded();
    let tx2 = tx.clone();
    let rx2 = rx.clone();
    let handle_a = thread::spawn(move || {
        for msg in rx {
            println!("Thread A: Received {}", msg);
        }
    });
    let handle_b = thread::spawn(move || {
        for msg in rx2 {
            println!("Thread B: Received {}", msg);
        }
    });
    for i in 0..10 {
        println!("Main thread: Sending {}", i);
        tx.send(i).unwrap();
        tx2.send(i).unwrap();
    }
    drop(tx);
    drop(tx2);
    handle_a.join().unwrap();
    handle_b.join().unwrap();

    println!("Main thread: Exiting.")
}
