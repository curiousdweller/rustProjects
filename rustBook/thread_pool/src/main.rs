use thread_pool::ThreadPool;
use std::thread;

fn main() {
let thread_pool = ThreadPool::new(3);
let foo = || {println!("WORKING In thread: {:?}", thread::current().id());
thread::sleep(std::time::Duration::from_secs(1));
};
// thread_pool.execute(foo.clone());
thread_pool.execute(foo.clone());
thread_pool.execute(foo.clone());
thread_pool.execute(foo.clone());
thread_pool.execute(foo.clone());
for thread in thread_pool.handles {
    thread.join().expect("broken here bro");
    
}
println!("YOYO WE DONE BRO");

thread::sleep(std::time::Duration::from_secs(10));
}