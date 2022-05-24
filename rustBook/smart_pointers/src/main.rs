use std::ops::Deref;
use std::rc::Rc;
use std::cell::{Ref, RefCell, RefMut};
use List::{Cons, Nil}; 
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("I have been dropped");
    }
}

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    let r = Rc::new(89900);
    let r1 = &r;
    println!("{}", r);

    let b1 = Box::new(String::from("HOWDY THERE"));
    let s = b1.deref();
    println!("{}", b1);

}

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil
}