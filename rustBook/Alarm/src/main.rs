use std::env;
fn main() {
    println!("Hello, world!");

    let args = env::args();
    println!("{:?}", args);

    let some = Some(String::from("hello"));
    let new = some.map(|s| {
        900
    });
    println!("{:?}", some);
}
