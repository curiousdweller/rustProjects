use std::{
    collections::HashMap,
    future::Future,
    mem,
    pin::Pin,
    sync::{
        mpsc::{channel, Sender},
        Arc, Mutex,
    },
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker, self},
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};
use MyFuturesImpl::{self, reactor::*, future::*};
fn main() {
    let start = Instant::now();
    let reactor = Reactor::new();
    println!("Hello, world!");
    let task1 = Task::new(reactor.clone(), 1, 8);
    let task2 = Task::new(reactor.clone(), 2, 14);

    let future1 = async {
        println!("Beginning task1 {}", task1.id);
        let id = task1.await;
        println!("completed task one {}", id);
    };


    let fut2 = async {
        let val = task2.await;
        println!("Got {} at time: {:.2}.", val, start.elapsed().as_secs_f32());
    };

    let main_future = async {
        future1.await;
        fut2.await;
        String::from("*********************************************************************")
    };

    println!("{}", MyFuturesImpl::block_on(main_future));
}
