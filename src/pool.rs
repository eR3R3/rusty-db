use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::sync::mpsc::*;
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<JoinHandle<()>>,
    sender: Sender<Job>
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        let (sender, receiver) = channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::new();
        for i in 0..size {
            let receiver = Arc::clone(&receiver);
            let worker = thread::spawn( move || {
                loop {
                    let job = receiver.lock().unwrap().recv();
                    match job {
                        Ok(job) => {
                            job()
                        },
                        Err(err) => {
                            println!("worker {} got shut down, err: {}", i, err);
                            break
                        }
                    }
                }
            });
            workers.push(worker);
        }
        Self {
            sender,
            workers
        }
    }

    pub fn execute<F>(&self, function: F)
    where F: FnOnce() + Send + 'static
    {
        self.sender.send(Box::new(function)).unwrap()
    }
}