pub fn is_palindrome(num: usize) -> bool {
    let s = num.to_string();
    let rev = s.chars().rev();
    let s1: String = rev.collect();
    println!("{}", s1);

    if s == s1 {
        true
    } else {
        false
    }
}
