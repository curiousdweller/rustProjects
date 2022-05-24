mod second;
use std::thread;
use std::time::Duration;
fn main() {

    second::multiple_producers();
    
    // thread::spawn(|| {
    //     for i in 1..1000 {
    //         println!("I has the value: {} IN SPAWNED THREAD ", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // for i in 1..10 {
    //     println!("I has the value:{} IN MAIN THREAD", i);
    //     thread::sleep(Duration::from_millis(10));
    // }
}
