use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess a Numer:");
    let rand_number = rand::thread_rng().gen_range(1..=100);
    loop {
        let mut guess =  String::new();
        println!("{}",rand_number);

    io::stdin() 
    .read_line(&mut guess)
    .expect("Error");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a number: {} is NAN! ", guess);
            continue;
        }
    };
    match guess.cmp(&rand_number) {
        Ordering::Equal => {
            println!("Well done you guessed {}!!!", rand_number);
            break;
        },
        Ordering::Less => println!("Too low"),
        Ordering::Greater => println!("Too high"),
    }
    }
    
    
    
}
// Implement a guesser for a number the user is thinking off