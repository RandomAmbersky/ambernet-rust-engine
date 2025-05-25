extern crate asn_core_bus;
extern crate tokio;

use crate::job::Job;
use crate::worker::Worker;
use asn_core_bus::AsnWorkerPool;
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::sync::{Mutex, mpsc};

pub struct TokioWorkerPool {
    rt: Runtime,
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl TokioWorkerPool {
    fn new(size: usize) -> Self {
        let rt = tokio::runtime::Runtime::new().unwrap();

        let (sender, rec) = mpsc::channel::<Job>(32);
        let receiver = Arc::new(Mutex::new(rec));

        let mut workers = Vec::with_capacity(10);

        for id in 0..size {
            let worker = Worker::new(id, receiver.clone());
            workers.push(worker);
        }

        TokioWorkerPool {
            rt,
            workers: vec![],
            sender: Some(sender),
        }
    }
}

impl<F> AsnWorkerPool<F> for TokioWorkerPool
where
    F: FnOnce(),
{
    fn run_main_thread(&self, func: F) -> Result<(), String> {
        todo!()
    }

    fn run_thread(&self, func: F) -> Result<(), String> {
        todo!()
    }
}

impl Drop for TokioWorkerPool {
    fn drop(&mut self) {
        if let Some(sender) = self.sender.take() {
            drop(sender);
        }

        self.rt
            .block_on({ tokio::time::sleep(std::time::Duration::from_secs(1)) });
    }
}

pub fn new_tokio_pool<F: FnOnce()>(size: usize) -> impl AsnWorkerPool<F> {
    TokioWorkerPool::new(size)
}
