use closure::{generate_workout, Cacher};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
fn main() {

    let intensity = 10;
    let random_number = 7;
    generate_workout(intensity  , random_number);

}


#[test]
fn call_with_different_values() {
    let mut c = Cacher{
        func:  |intensity| {
            println!("Doing some expensive calculation");
            thread::sleep(Duration::from_secs(2));
            intensity
        },
        intensity: HashMap::new()
    };

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_ne!(v2, 2);
}