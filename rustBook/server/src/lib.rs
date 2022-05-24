use std::mem::take;
use std::thread;
use std::sync::{mpsc, Mutex, Arc};

type Job = Box<dyn FnOnce() ->() + 'static + Send>;
pub struct ThreadPool{
    workers: Vec<Option<Worker>>,
    sender: mpsc::Sender<Message>
}

impl ThreadPool {
           
        /// Create a new thread pool where size is the number of threads
        /// # Panics
        /// new function panics if size = 0
    pub fn new(size: usize) -> Self {
        assert_ne!(size, 0);

        let (sender, receiver) = mpsc::channel::<Message>();
        let receiver = Arc::new(Mutex::new(receiver));
        
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            let worker = Worker::new(id, Arc::clone(&receiver));
            workers.push(Some(worker));
        }
        ThreadPool {workers , sender}
    }
    pub fn execute<F>(&self, work: F) 
        where F: FnOnce() + Send + 'static {
            let job = Box::new(work);
            self.sender.send(Message::Job(job)).unwrap();
    }

}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in & mut self.workers {
        println!("Shutting down {}", worker.as_ref().unwrap().id);
        if let Some(w) = worker.take() {
            w.thread.join().unwrap();
        }
    }
    }
}


struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
        let thread = thread::spawn(move || {
            'outer :loop {
            if let Message::Job(job) = (*receiver).lock().unwrap().recv().unwrap() {
            job();
            } else {
                break 'outer ;
            }
        }
        });
        Self {
            id,
            thread
        }
    }

    fn join(self) {
        self.thread.join().unwrap();
    }
}

enum  Message {
    Job(Job),
    None
}
