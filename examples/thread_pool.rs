use anyhow::Result;
use crossbeam_channel as mpsc;
use std::{
    mem,
    sync::{Arc, Mutex},
    thread::{self, sleep},
    time::Duration,
};

type Job = Box<dyn FnOnce() + Send + 'static>;

#[allow(dead_code)]
struct ThreadPool {
    workers: Vec<thread::JoinHandle<()>>,
    sender: Option<mpsc::Sender<Box<dyn FnOnce() + Send + 'static>>>,
}

impl ThreadPool {
    fn new(name: &str, size: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::bounded::<Job>(128);
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for i in 1..=size {
            let name = format!("{}-{}", name, i);
            let receiver = Arc::clone(&receiver);
            // spawn with name
            let handle = thread::Builder::new()
                .name(name)
                .spawn(move || loop {
                    let Ok(guard) = receiver.lock() else {
                        break;
                    };
                    let Ok(job) = guard.recv() else {
                        break;
                    };
                    job();
                })
                .unwrap();
            workers.push(handle);
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    fn execute<F>(&self, f: F) -> Result<()>
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        let Some(ref sender) = self.sender else {
            return Err(anyhow::anyhow!("ThreadPool is shutdown"));
        };
        let Ok(_) = sender.send(job) else {
            return Err(anyhow::anyhow!("failed to send job to worker thread"));
        };
        Ok(())
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        let sender = mem::take(&mut self.sender);
        if let Some(sender) = sender {
            drop(sender);
        }
    }
}

fn main() -> Result<()> {
    let pool = ThreadPool::new("worker", 4);
    pool.execute(|| {
        println!("Hello from the thread pool!");
    })?;
    pool.execute(|| {
        println!("More tasks can be processed.");
    })?;

    sleep(Duration::from_secs(1));

    Ok(())
}
