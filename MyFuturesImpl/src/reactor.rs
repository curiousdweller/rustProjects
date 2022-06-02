use core::panic;
use std::{
    collections::HashMap,
    sync::{
        mpsc::{channel, Sender},
        Arc, Mutex,
    },
    task::Waker,
    thread::{self, JoinHandle},
    time::Duration,
};
#[derive(Debug)]
pub enum TaskState {
    Ready,
    // Waker used to notify the exector a task is ready
    NotReady(Waker),
    Finished,
}

#[derive(Debug)]
enum Event {
    Timeout(u64, usize),
    Close,
}
#[derive(Debug)]
pub struct Reactor {
    channel: Sender<Event>,
    handle: Option<JoinHandle<()>>,
    pub tasks: HashMap<usize, TaskState>,
}

impl Reactor {
    pub fn new() -> Arc<Mutex<Box<Reactor>>> {
        let (sender, recvr) = channel::<Event>();
        let reactor = Arc::new(Mutex::new(Box::new(Reactor {
            channel: sender,
            handle: None,
            tasks: HashMap::new(),
        })));

        let reactor_clone = Arc::downgrade(&reactor);

        // Spawn the reactor thread
        let handle = thread::spawn(move || {
            // Spawn a thread for each task?? , track the handles
            let mut handles: Vec<JoinHandle<()>> = vec![];

            // Receive a message from the main thread and act upon it
            for message in recvr {
                let reactor_clone = reactor_clone.clone();
                match message {
                    Event::Close => break,
                    Event::Timeout(seconds, task_number) => {
                        let event_handle = thread::spawn(move || {
                            thread::sleep(Duration::from_secs(seconds));
                            // Should never error, thread is joined before main is terminated.
                            let reactor = reactor_clone.upgrade().unwrap();
                            reactor
                                .lock()
                                .map(|mut r| {
                                    // map on results are useful becuase you can operate directly on the inner type if
                                    // the value is Ok(_)
                                    r.wake(task_number);
                                })
                                .unwrap();
                        });
                        handles.push(event_handle);
                    }
                }
            }
            for handle in handles {
                handle.join().unwrap();
            }
        });
        reactor
            .lock()
            .map(|mut r| {
                r.handle = Some(handle);
            })
            .unwrap();
        reactor
    }
    pub fn register(&mut self, time: u64, waker: Waker, id: usize) {
        if self.tasks.insert(id, TaskState::NotReady(waker)).is_some() {
            panic!("Tried to register the same event");
        } else {
            self.channel.send(Event::Timeout(time, id)).unwrap();
        }
    }

    fn wake(&mut self, id: usize) {
        // After waking the task should be put into the Ready state
        match self.tasks.insert(id, TaskState::Ready).unwrap() {
            TaskState::NotReady(waker) => waker.wake(),
            TaskState::Finished => panic!("Tried to Wake task twice"),
            _ => unreachable!(),
        }
    }

    fn is_ready(&self, id: usize) -> bool {
        if let Some(status) = self.tasks.get(&id) {
            match status {
                TaskState::Ready => return true,
                _ => return false,
            }
        } else {
            panic!("Task with id {} does not exist", id);
        }
    }
}

impl Drop for Reactor {
    fn drop(&mut self) {
        // Because recvr.recv() is a blocking call and we are doing it in a loop in the reactor
        // thread, if we were to join it before closing the channel it would block until the channel closes (when main() ends, worst case).
        self.channel
            .send(Event::Close)
            .expect("Error closing channel to reactor-thread");
        self.handle
            .take()
            .unwrap()
            .join()
            .expect("Error joining reactor thread");
    }
}
