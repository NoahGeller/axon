use std::thread::{self, JoinHandle};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => job(),
                Message::Terminate => break
            }
        });

        Worker { thread: Some(thread) }
    }
}

enum Message {
    NewJob(Job),
    Terminate
}

/// A basic threadpool type.
///
/// Sends jobs over a channel to be executed by a patient pool of threads.
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn new(num_threads: usize) -> ThreadPool {
        assert!(num_threads > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(num_threads);

        for _ in 1..num_threads {
            workers.push(Worker::new(receiver.clone()));
        }

        ThreadPool { workers, sender }
    }
    /// Sends a job to be executed by one of the threads in the pool.
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
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
