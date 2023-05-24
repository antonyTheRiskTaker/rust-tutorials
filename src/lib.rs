use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    /// 
    /// The size is the number of threads in the pool.
    /// 
    /// # Panics
    /// 
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            // Create some threads and store them in the vector
        }

        ThreadPool { threads }
    }

    // TODO: continue from `A Worker Struct Responsible for Sending Code from the ThreadPool to a Thread`

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {}
}