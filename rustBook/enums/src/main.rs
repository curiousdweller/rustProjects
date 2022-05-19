struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn square(bottom_right: Point, len: f32) -> Rectangle {
        let top_left = Point{x: bottom_right.x - len, y: bottom_right.y + len};
        Rectangle{top_left, bottom_right}
    }
}

fn main() {
    enum IpAddr{
        V4(String),
        V6(String)
    }
    #[derive(Debug)]
    enum Coin{
        Pound,
        Penny,
        Fiver,
        Tenner
    }
    #[derive(Debug)]
    enum Option<t> {
        Some(t),
        None
    }

    impl Coin {
        fn coin_in_pence(coin: Coin) -> u32 {
            match coin{
                Coin::Fiver => 500,
                Coin::Pound => 100,
                Coin::Penny => 1,
                Coin::Tenner => 1000
            }
        }
    }

    let num = Coin::coin_in_pence(Coin::Tenner);
    println!("{}", num);

    fn add_to_optional(num: Option<i32>) -> Option<i32> {
        match num {
            Option::None => Option::None,
            Option::Some(i) => Option::Some(i + 1)
        }
    }
    
    let num = add_to_optional(Option::Some(300));
    println!("{:?}",num);
}
//  Enums are cool!