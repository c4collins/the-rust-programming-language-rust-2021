use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

type Job = Box<dyn FnOnce() + Send + 'static>;
type WorkerReceiver = Arc<Mutex<mpsc::Receiver<Job>>>;

#[derive(Debug)]
pub enum ThreadPoolError {
    LessThanOne,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// Create a ThreadPool
    ///
    /// The size is the number of threads in the pool
    pub fn build(size: usize) -> Result<ThreadPool, ThreadPoolError> {
        if size < 1 {
            return Err(ThreadPoolError::LessThanOne);
        }
        Ok(ThreadPool::new(size))
    }

    fn new(size: usize) -> Self {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        Self {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: WorkerReceiver) -> Self {
        let builder = thread::Builder::new();
        let handle = builder
            .spawn(move || loop {
                let job = receiver
                    .lock()
                    .expect("Failed to get ThreadPool Job lock receiver on Worker")
                    .recv();
                match job {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");
                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; Shutting down.");
                        break;
                    }
                }
            })
            .unwrap();

        Worker {
            id,
            thread: Some(handle),
        }
    }
}
