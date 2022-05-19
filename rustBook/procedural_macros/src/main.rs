use procedural_macros::{my_attribute, do_nothing};

fn main() {
    do_nothing!(let suii = 900;);
    let sui = 9000;  
    #[procedural_macros::my_attribute(lol)]
    fn bar() {
        println!("Hello there partner");
    }
    bar();

}
