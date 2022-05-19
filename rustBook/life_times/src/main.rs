fn main() {
    // let s1 = String::from("Hello sir, welcome to rust");
    // let s2 = "Shorter_Stirng";



    // fn longest<'a>(x: &'a str , y: &'a str) -> &'a str {
    //     if x.len() > y.len() {
    //         return x
    //     } 
    //     y
    // }

    // println!("{} is the longest string", longest(s1.as_str(), s2));
    // println!("{}", s1);
    let a;
    {
        let b = String::from("heellooo");
        a = &b;
    }

    

}
