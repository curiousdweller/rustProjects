use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::sync::Mutex;
use std::sync::Arc;



pub fn second() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("howdy partner");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", &received);
}

pub fn multiple_producers() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }


}

pub fn mutex_example() {
    let m = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 1..=10 {
        let counter = Arc::clone(&m);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }



}