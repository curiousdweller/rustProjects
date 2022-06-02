use crate::reactor::{Reactor, TaskState};
use std::{
    future::Future,
    pin::Pin,
    sync::{
        Arc, Mutex,
    },
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
    thread,
};
// Some notes on why use arc::into_raw and arc::from_raw
/*
When turning an Arc<T> into *const T the data that keeps track of how many references there are to T still exists,
but it is ignored when working with the raw pointer and its as if you are operating on T directly
So When Going from T to Arc<T>, when Arc<T> is dropped the reference count is decremented, but if *const T is dropped
the reference count is not decremented.
*/
type SafeReactor = Arc<Mutex<Box<Reactor>>>;
#[derive(Debug)]
pub struct Task {
    pub id: usize,
    pub data: u64,
    pub reactor: Arc<Mutex<Box<Reactor>>>,
}

#[derive(Debug)]
pub struct MyWaker {
    pub thread: thread::Thread,
}

// fn mywaker_clone(s: &MyWaker) -> RawWaker {
//     let arc = unsafe { Arc::from_raw(s) };
//     std::mem::forget(arc.clone()); // increase ref count
//     RawWaker::new(Arc::into_raw(arc) as *const (), &VTABLE)
// }

pub fn mywaker_into_waker(s: *const MyWaker) -> Waker {
    let raw_waker = RawWaker::new(s as *const (), &VTABLE);
    unsafe { Waker::from_raw(raw_waker) }
}

#[allow(dead_code)]
const VTABLE: RawWakerVTable = unsafe {
    RawWakerVTable::new(
        |s| mywaker_clone(&*(s as *const MyWaker)),   // clone
        |s| mywaker_wake(&*(s as *const MyWaker)),    // wake
        |s| (*(s as *const MyWaker)).thread.unpark(), // wake by ref (don't decrease refcount) --> Operating on T as if it was just T; no notion of a ref count.
        |s| drop(Arc::from_raw(s as *const MyWaker)), // decrease refcount
    )
};

// Turn into arc so the reference count can be decremented when ```waker_arc``` goes out of scope (Dropped)
fn mywaker_wake(s: &MyWaker) {
    let waker_ptr: *const MyWaker = s;
    let waker_arc = unsafe { Arc::from_raw(waker_ptr) };
    waker_arc.thread.unpark();
}

fn mywaker_clone(s: &MyWaker) -> RawWaker {
    let arc = unsafe { Arc::from_raw(s) };
    std::mem::forget(arc.clone()); // increase ref count
    RawWaker::new(Arc::into_raw(arc) as *const (), &VTABLE)
}

impl Task {
    pub fn new(reactor: SafeReactor, id: usize, data: u64) -> Self {
        Task { reactor, id, data }
    }
}

impl Future for Task {
    type Output = usize; //The Task ID

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Query the reactor to get the state of this task
        let mut reactor = self.reactor.lock().unwrap();
        match reactor.tasks.get(&self.id) {
            Some(task) => match task {
                TaskState::Ready => {
                    reactor.tasks.insert(self.id, TaskState::Finished);
                    return Poll::Ready(self.id);
                }
                TaskState::NotReady(_) => {
                    reactor
                        .tasks
                        .insert(self.id, TaskState::NotReady(cx.waker().clone()));
                    return Poll::Pending;
                }
                TaskState::Finished => panic!("Polled A finished Task with id {}", self.id),
            },
            None => {
                reactor.register(self.data, cx.waker().clone(), self.id);
                return Poll::Pending;

                // Todo, register on reactor and return pending
            }
        }
    }
}
