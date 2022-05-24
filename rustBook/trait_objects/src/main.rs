fn main() {
    println!("Hello, world!");
    // Show the difference between a struct that uses genereics
    // in a vector and a struct that uses trait objects.
}


struct Example<T> {
    v: Vec<T>,
}
// This struct can only create a vec of ONE CONCRETE type

struct BetterExample {
    v: Vec<Box<dyn Draw>>
}
// This struct can create a vector of ANY type that implements the Draw trait
pub trait Draw {
    fn Draw(&self);
}

// This function will be implemented uniquely for each given type that calls it.
fn testing<T: Draw>(param:T) {
    todo!()
}

// This function is implemented using a vtable
fn testing_with_trait_object(param: Box<dyn Draw>) {
    todo!()
}