use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

pub struct PoolCreationError {
    text: String,
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {id} got job. Executing...");

                job()
            }
        });

        Worker { id, thread }
    }
}

impl PoolCreationError {
    fn new() -> PoolCreationError {
        PoolCreationError {
            text: String::from("Pool Creation via build() is failed"),
        }
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        for i in 0..size {
            let worker = Worker::new(i, Arc::clone(&receiver));
            workers.push(worker);
        }

        ThreadPool { workers, sender }
    }

    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        match size {
            s if s <= 0 => return Err(PoolCreationError::new()),
            s if s > 0 => {
                let mut workers = Vec::with_capacity(size);
                let (sender, receiver) = mpsc::channel();
                let receiver = Arc::new(Mutex::new(receiver));
                for i in 0..size {
                    let worker = Worker::new(i, Arc::clone(&receiver));

                    workers.push(worker);
                }

                return Ok(ThreadPool { workers, sender });
            }
            _ => return Err(PoolCreationError::new()),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}
