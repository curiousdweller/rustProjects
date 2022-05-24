fn main() {

    macro_rules! set {
        ($i: ident) => {
            let $i = 300;
        };
    }

    
    let x = 5.0 / 2.00 ;
    println!("{}", x);
    let days_of_week = [
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday"
    ];

    println!("{}", days_of_week[2]);

    let f = 44;
    let rf = &f;
    let rrf = &rf;
    println!("{}", rrf);

    let a = [23, 44, 45];
    let a1 = &a[..2];
    set!(sigm);
    println!("{}", sigm);
}


