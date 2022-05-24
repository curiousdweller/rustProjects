use unsafe_rust::*;
fn main() {
    let mut x = vec![1,2,3,4,5];
    let x1 = &x as *const Vec<i32>;
    let x2 = &mut x as *mut Vec<i32>;
    let x3 = &x;
    println!("{:?}, {:?}, {:?}", x1, x2, x3);

    unsafe {
    
        println!("{:?}", *x1);
        drop(x);
        println!("{:?}", *x2);
    }

    let mut slice = vec![1,2,3,4,5,6];
    let r = &mut slice[..];
    let (first, second) = r.split_at_mut(3);
    println!("{:?}, {:?}", first, second);



}