use std::thread::JoinHandle;

pub struct ThreadPool;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        ThreadPool
    }

    pub fn execute<F, T>(&self, _f: F) -> JoinHandle<T>
    where
        F: FnOnce() + Send + 'static,
        T: Send + 'static,
    {
    }
    
}