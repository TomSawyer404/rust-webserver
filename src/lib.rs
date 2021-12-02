use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    /// Create a threadpool
    ///
    /// # Panics
    ///
    /// `new(num: size)` will panic when `num` is `0`
    pub fn new(num: usize) -> Self {
        assert!(num > 0);

        let mut workers = Vec::with_capacity(num);
        for id in 0..num {
            // create some threads and store them in the vector
            workers.push(Worker::new(id));
        }

        ThreadPool { workers }
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
    fn new(num: usize) -> Self {
        let thread = thread::spawn(|| {});
        Worker {
            id: (num),
            thread: (thread),
        }
    }
}
