use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::{Mutex, Arc};
use std::rc::Rc;

fn simple_one() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
}

/*
fn shared_mutex() {
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
}
*/

/*
fn not_fixed_shared_mutex() {
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 1..10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn( move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
*/

fn fixed_shared_mutex() {
    let counter = Arc::new(Mutex::new(10));
    let mut handles = vec![];

    for _ in 1..10 {
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

    println!("Result: {}", *counter.lock().unwrap());
}

pub fn example_mutex() {
    //simple_one();
    //shared_mutex();
    //not_fixed_shared_mutex();
    fixed_shared_mutex();
}
