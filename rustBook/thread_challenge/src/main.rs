use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::sync::mpsc;
//  A channel is useful when using many threads to do some work individually
// (multiple workes) and then allowing the main thread to synthesize this work
// (single manager)

fn main() {
    println!("Hello, world!");
    let v = vec!["Hello", "You know", "Bye", "Hello", "You know", "Bye", "Hello", "You know", "Bye", "Hello", "You know", "Bye"];
    let map = count(v.as_slice(), 10);
    println!("{:?}", map);

}

// Want a func that takes an array string slices, iterates through
// And counts the occurence of each letter

fn count(words: &[&str], workers: usize) -> HashMap<char, u32> {
    let mut map = HashMap::new();
    let chunks = words.chunks((words.len() / workers).max(1));
    let (sender, receiver) = mpsc::channel();
    // If there are more workers than len, then only use 1 chunk --> cannot have 0.8 of a chunk

    for chunk in chunks {
        let temp_sender = sender.clone();
        let string = chunk.join("");
        let handle = thread::spawn(move || {
            let mut map = HashMap::new();
            for char in string.chars() {
                let count = map.entry(char).or_insert(0);
                *count += 1
            }
            temp_sender.send(map).unwrap();

        });
        }
        drop(sender);
        for rec in receiver {
            for (key, value) in rec {
                let count = map.entry(key).or_insert(0);
                *count += 1;
            }
        }
        map
    }
