// use std::{collections::HashMap, ascii::AsciiExt};
// fn main () {
//     let v = vec!["hello", "hi", "hey"];
//     let _ = frequency(&v, 0);
//     fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
//         let mut map = HashMap::new();

//         for word in input {
//             for char in word.chars().filter(|c| {
//                 c.is_alphabetic()
//             }) {
//                 *map.entry(char.to_ascii_lowercase()).or_insert(0) += 1;
//             }

//             println!("{:?}", map);
//         }
//         map

//     }

// }
use std::{collections::HashMap, hash::Hash};
use std::env;

fn main() {
    println!("Hello world!");
    let iter = env::args().into_iter();
    let collection: Vec<String> = iter.collect();
    println!("{:#?}", collection);

}

#[test]
fn balances_test() {
    let map = HashMap::<u32, u32>::new();
    assert_eq!(map.get(&100), None);
}

struct Balances {
    balances: HashMap<u32, u32>,
}

impl Balances {
    pub fn new() -> Self {
        Balances { balances: HashMap::new() }
    }

    pub fn transfer(&mut self, to: u32, from:u32, amount: u32) -> Result<(), &'static str> {
        let from_bal = *(self.balances.get(&from).ok_or("Non existent account!")?);
        Ok(())
    }
}