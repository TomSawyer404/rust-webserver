use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    /// Create a threadpool
    ///
    /// # Panics
    ///
    /// `new(num: size)` will panic when `num` is `0`
    pub fn new(num: usize) -> Self {
        assert!(num > 0);
        let mut threads = Vec::with_capacity(num);
        for _ in 0..num {
            // create some threads and store them in the vector
        }

        ThreadPool { threads }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
