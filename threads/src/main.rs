use std::thread;
use std::time::Duration;

pub fn thread_method() {
    for ii in 1..10 {
        println!("thread method: {}", ii);
        thread::sleep(Duration::from_millis(1500));
    }
}

fn run_threads() {

    let thr = thread::spawn( || {
            //thread_mehtod();
            for ii in 1..10 {
                println!("thread method: {}", ii);
                thread::sleep(Duration::from_millis(1500));
            }
        }
    );

    for ii in 1..10 {
        println!("Main thread {}", ii);
        thread::sleep(Duration::from_millis(1000));
    }

    // this guarantee to finish the thread:
    thr.join().unwrap();
}

fn run_threads2() {
    let v = vec![1, 2, 4];

    // we need to move variables because we have to remove reference to variables
    // from main thread
    let handle = thread::spawn( move || {
        for ii in 1..10 {
            println!("hello from thread: {} {}", ii, v[0]);
            thread::sleep(Duration::from_millis(1500));
        }
    });

    handle.join().unwrap();
}

fn main() {
    println!("Hello, world!");

    //run_threads();
    run_threads2();
}
