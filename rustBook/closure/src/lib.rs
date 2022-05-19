use std::{thread};
use std::time::Duration;
use std::collections::HashMap;


pub fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_result = |intensity| {
        println!("Doing some expensive calculation");
        thread::sleep(Duration::from_secs(2));
        intensity
    };
    let mut cache = Cacher{func: expensive_result, intensity: HashMap::new()};
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            cache.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            cache.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                cache.value(intensity)
            );
        }
    }
}

pub struct Cacher<T: Fn(u32) -> u32> {
    pub func: T,
    pub intensity: HashMap<u32,u32>
}

impl<T> Cacher<T> 
    where T: Fn(u32) -> u32 {
    pub fn value(&mut self, arg: u32) -> u32 {
        match (self.intensity).get(&arg) {
            Some(x) => *x,
            None => {
                (self.intensity).insert(arg, (self.func)(arg));
                *(self.intensity).get(&arg).unwrap()
            }
        }
    }
}

