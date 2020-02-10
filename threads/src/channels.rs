use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn simple_channel() {
    // multiple producer, single consumer
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();

    println!("Got: {}", received);
}

fn few_messages() {
    let (tx, rx) = mpsc::channel();

    let vals = vec![1, 2, 3, 4, 5];

    thread::spawn(move || {
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1000));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn cloning_producer() {
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![1, 2, 3, 4, 5];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1000));
        }
    });

    thread::spawn(move || {
        let vals = vec![6, 7, 8, 9, 10];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(1000));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

pub fn create_channel() -> Result<i32, &'static String> {
    //simple_channel();
    //few_messages();
    cloning_producer();
    return Ok(0);
}
