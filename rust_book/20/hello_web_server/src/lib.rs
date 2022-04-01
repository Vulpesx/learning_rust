use std::thread::{self, JoinHandle};
use std::sync::{mpsc, Arc, Mutex};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>
}

type Job = Box<dyn FnOnce() + Send + 'static>;
enum Message {
    NewJob(Job),
    Terminate,
}

impl ThreadPool {
    /// Creates a new ThreadPool with a defined number of
    /// threads.
    ///
    /// # Panics
    ///
    /// the `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, reciever) = mpsc::channel();

        let reciever = Arc::new(Mutex::new(reciever));

        let mut workers = Vec::with_capacity(size);

        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&reciever)));
        }

        ThreadPool { workers, sender }
    }

    //TODO: implement execute function.
    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
        {
            let job = Box::new(f);

            self.sender.send(Message::NewJob(job)).unwrap();
        }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Terminating all workers");

        for worker in &mut self.workers {
            println!("shutting down worker: {}", worker.id);

            if let Some(thread) = worker.handle.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    handle: Option<JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, reciever: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let handle = thread::spawn(move || loop {
            let msg = reciever.lock().unwrap().recv().unwrap();

            match msg {
                Message::NewJob(job) => {
                    println!("Worker: {} recieved a job; executing.", id);

                    job();
                }
                Message::Terminate => {
                    println!("Worker: {} recieved terminate command.", id);

                    break;
                }
            }
        });

        Worker {
            id,
            handle: Some(handle)
        }
    }
}

