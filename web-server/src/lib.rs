use std::sync::{mpsc, Arc, Mutex};
use std::thread;

#[derive(Debug)]
pub enum PoolCreationError {
    SizeOfZero,
}

enum Message {
    NewJob(Job),
    Terminate,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Drop for ThreadPool {
    fn drop(&mut self) {

        let mut counter = 0;
        while &counter < &self.workers.len() {
            self.sender.send(Message::Terminate).unwrap();
            counter += 1;
        }

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            } else {
                println!("Worker already terminated!");
            }
        }
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        match size {
            0 => Err(PoolCreationError::SizeOfZero),
            _ => {
                let (sender, receiver) = mpsc::channel();
                let receiver = Arc::new(Mutex::new(receiver));
                let mut workers = Vec::with_capacity(size);
                for id in 0..size {
                    workers.push(Worker::new(id, Arc::clone(&receiver)));
                }

                Ok(ThreadPool { workers, sender })
            }
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                }
            }
        });

        Worker { id, thread:Some(thread) }
    }
}
