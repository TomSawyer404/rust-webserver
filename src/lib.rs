use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

struct Job;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    /// Create a threadpool
    ///
    /// # Panics
    ///
    /// `new(num: size)` will panic when `num` is `0`
    pub fn new(num: usize) -> Self {
        assert!(num > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(num);
        for id in 0..num {
            // create some threads and store them in the vector
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(num: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(|| {
            receiver;
        });

        Worker {
            id: (num),
            thread: (thread),
        }
    }
}
