use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

#[derive(Debug)]
pub enum PoolCreationError {
    SizeOfZero,
}

impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        match size {
            0 => Err(PoolCreationError::SizeOfZero),
            _ => {
                let mut threads = Vec::with_capacity(size);
                for _ in 0..size {
                    // create some threads and store them in the vector
                }
        
                Ok(ThreadPool { threads })
            },        
        }
    }
    
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static {

    }
}