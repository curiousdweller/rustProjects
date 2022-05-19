use std::sync::{Arc, Mutex};
use std::thread;
pub fn sleep_sort(nums: [u32; 5]) {
    let mut threads = vec![];
    let res: Vec<u32> = vec![];
    let mut_result = Mutex::new(res);
    let arc_result = Arc::new(mut_result);

    for i in nums {
        let result = Arc::clone(&arc_result);
        // The first returned thread is the one with the 3, and will be the first to return
        let thread = thread::spawn(move || {
            thread::sleep(std::time::Duration::from_secs(i.into()));
            println!("{}", i);
            let mut_result = &mut *(&*result).lock().unwrap();
            mut_result.push(i);
        });
        threads.push(thread);
    }
    for thread in threads {
        thread.join().unwrap();
    }

    println!("{:?}", arc_result);
}
