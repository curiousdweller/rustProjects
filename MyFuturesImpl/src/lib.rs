use std::{
    future::Future,
    sync::Arc,
    pin::Pin, 
    task::{Context, Poll},
    thread
};
pub mod reactor;

pub mod future;


// An Executor polls a future. One of three things can occur after polling:
// 1. It (Future) returns Poll::Ready and the future can be executed
// 2. It returns Poll::Pending and progress on the future cannot be made
// 3. It hasn't been polled before, thus a waker is passed into the future.


pub fn block_on<F: Future>(mut future: F) -> F::Output {
    // Construct a waker to pass to Future::Poll, the waker will 
    // Allow the reactor to "notify" the executor by unparking the thread
    // when the task is ready
    let my_waker = Arc::new(future::MyWaker{
        thread: thread::current()
    });
    let waker = future::mywaker_into_waker(Arc::into_raw(my_waker)); 
    let mut cx = Context::from_waker(&waker);

    let mut future = unsafe { Pin::new_unchecked(&mut future) };

    let val = loop {
        match Future::poll(future.as_mut(), &mut cx) {
            Poll::Ready(num) => break num,
            Poll::Pending => thread::park()
        }
    };

    val

    // Need to do some park nonsense here...

}