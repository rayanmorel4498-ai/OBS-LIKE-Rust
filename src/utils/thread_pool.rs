// visualisation_module/src/utils/thread_pool.rs

use std::sync::{Arc, Mutex, Condvar};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    handle: Option<thread::JoinHandle<()>>,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    queue: Arc<Mutex<Vec<Job>>>,
    cvar: Arc<Condvar>,
    running: Arc<Mutex<bool>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        let queue: Arc<Mutex<Vec<Box<dyn FnOnce() + Send + 'static>>>> = Arc::new(Mutex::new(Vec::new()));
        let cvar = Arc::new(Condvar::new());
        let running = Arc::new(Mutex::new(true));

        let mut workers = Vec::with_capacity(size);

        for _ in 0..size {
            let q = Arc::clone(&queue);
            let cv = Arc::clone(&cvar);
            let r = Arc::clone(&running);

            let handle = thread::spawn(move || {
                loop {
                    let job = {
                        let mut lock = q.lock().unwrap();
                        while lock.is_empty() && *r.lock().unwrap() {
                            lock = cv.wait(lock).unwrap();
                        }
                        if !*r.lock().unwrap() {
                            return;
                        }
                        lock.pop()
                    };

                    if let Some(job) = job {
                        job();
                    }
                }
            });

            workers.push(Worker {
                handle: Some(handle),
            });
        }

        Self {
            workers,
            queue,
            cvar,
            running,
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let mut lock = self.queue.lock().unwrap();
        lock.push(Box::new(f));
        self.cvar.notify_one();
    }

    pub fn shutdown(&mut self) {
        *self.running.lock().unwrap() = false;
        self.cvar.notify_all();

        for worker in &mut self.workers {
            if let Some(h) = worker.handle.take() {
                let _ = h.join();
            }
        }
    }
}
