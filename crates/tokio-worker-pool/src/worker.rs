use std::sync::Arc;
use crate::job::Job;

use tokio::{
    sync::{mpsc, Mutex},
    task::JoinHandle,
};

pub struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn take_thread(&mut self) -> Option<JoinHandle<()>> {
        self.thread.take()
    }

    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = tokio::spawn(async move {
            loop {
                let task = {
                    let mut guard = receiver.lock().await;
                    guard.recv().await
                };

                match task {
                    Some(job) => {
                        println!("Worker {id} got a job; executing.");
                        job(id.clone());
                        println!("Worker {id} job ending.");
                    }
                    None => {
                        println!("Error. Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
