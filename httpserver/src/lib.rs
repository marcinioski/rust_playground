use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

type AJob = Arc<Mutex<mpsc::Receiver<Message>>>;

enum Message {
    NewJob(Job),
    Terminate,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new (id: usize, receiver: AJob) -> Worker {
        let thread = thread::spawn( move || {
            loop {
                if let Ok(message) = receiver.lock().unwrap().recv() {
                    let message = message;
                    match message {
                        Message::NewJob(job) => job(),
                        Message::Terminate => break,
                    }
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

pub enum CreationError {
    WrongSize,
    SystemError(i32),
    Other,
}

impl ThreadPool {
    fn gen_channels() -> (mpsc::Sender<Message>, mpsc::Receiver<Message>) {
        mpsc::channel()
    }

    pub fn new(pool_size: usize) -> Result<ThreadPool, CreationError> {
        if pool_size > 0 {
            let mut workers = Vec::with_capacity(pool_size);

            let (sender, receiver) = ThreadPool::gen_channels();
            let receiver = Arc::new(Mutex::new(receiver));

            for id in 0..pool_size {
                workers.push(Worker::new(id, Arc::clone(&receiver)));
            }

            Ok(ThreadPool { workers, sender } )
        }
        else
        {
            Err(CreationError::WrongSize)
        }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
