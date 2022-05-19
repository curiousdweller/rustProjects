use core::panic;

fn main() {
    use std::collections::HashMap;
    let mut es = String::from("hello");

    let r1 = &mut es;
    let mut r2: String = r1.clone();
    r2.push_str("Hello");

    println!("{}, {}", r2, es);

    let mut k = HashMap::new();
    k.insert(es, 50);
    k.entry(String::from("YOO")).or_insert(9999);
    println!("{:#?}", k);

    let mut v = vec![1, 2, 3];
    let V = v[2] + 20;
    let eV = &mut v[2];
    v[2]  += 30;
    println!("{:?}", v);



    let a = 32;


    // let s = String::from("hello world");

    // let s_r: &str = "hello";
    // // STR DOESNT HAVE KNOWN SIZE AT COMPILE TIME BUT &STR DOES BECAUSE ITS JUST A POINTER AND CAPACITY!!!

    // let hello = &s[0..5];
    // let world = &s[6..11];

    // struct Rectangle {
    //     length: i32,
    //     width: i32
    // }

    // impl Rectangle {
    //     fn can_hold(&self, rect: &Rectangle) -> bool {
    //         self.width > rect.width && self.length > rect.length
    //     }
    // }

    // let rect_2 = Rectangle{length: 20, width: 30};
    // let rectangle = Rectangle{length: 344, width: 4000};
    // println!("{}", rectangle.width);
    // println!("Rectangle can hold rect_2? {}", rectangle.can_hold(&rect_2));
} // Here, some_integer goes out of scope. Nothing special happens.

