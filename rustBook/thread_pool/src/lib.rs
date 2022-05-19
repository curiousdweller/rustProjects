use std::process::id;
use std::thread;
use std::sync::{mpsc, Mutex, Arc};

pub struct ThreadPool {
    pub handles: Vec<thread::JoinHandle<()>>,
    sender: mpsc::Sender<Box<dyn Fn() + Send>>,
}


impl ThreadPool {
    pub fn new(num: u8) -> Self {
        let (tx, rx) = mpsc::channel::<Box<dyn Fn() + Send>>();
        let rx = Arc::new(Mutex::new(rx));
        let mut _handles = std::vec::Vec::new();

        for _ in 0..num {

            let rx = rx.clone();
            let _handle = thread::spawn(move | | 
                
                loop {
                    let work = (rx).lock().expect("Hunch was right").recv().unwrap();
                    // let work  = match work {
                    //     Ok(_work) => _work,
                    //     Err(_) => break
                    // };
                    println!("Starting work {:?}", thread::current().id());
                    work();
                    println!("Finsishing Work {:?}", thread::current().id());
                
            });

            _handles.push(_handle);   

        }
        Self {
            handles: _handles,
            sender: tx
        }
    }

    pub fn execute<T>(&self, work: T) 
    where T: Fn() + Send + 'static + Clone
    {
        self.sender.send(Box::new(work.clone())).expect("Sender is messing up");

    }
    
    
}

 

#[cfg(test)]  
mod tests {
    use crate::ThreadPool;
    use std::thread;

    #[test]
    fn it_works() {
        let thread_pool = ThreadPool::new(10);
        let foo = || {println!("WORKING In thread: {:?}", thread::current().id())};
        // thread_pool.execute(foo.clone());
        thread_pool.execute(foo);
    }
}
