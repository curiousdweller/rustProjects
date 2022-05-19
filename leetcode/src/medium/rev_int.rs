pub fn rev_int(i: i32) -> i32 {
    let mut p: i32;
    // Need to deal with case where x < 0
    let s = i.to_string();
    if i < 0 {
        let mut iterator = s.chars();
        let minus_sign = iterator.next().unwrap().to_string();
        let str: String = iterator.rev().collect();
        p = match format!("{}{}", minus_sign, str).parse() {
            Ok(n) => n,
            Err(_) => 0
        };


    }
    else {
    let s: String = s.chars().rev().collect();
         p = match s.parse() {
            Ok(n) => n,
            Err(_) => 0
        };
    }
    p
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn overflow() {
        assert_ne!(rev_int(2147483647), 0);
    }
    #[test]
    fn minus() {
        assert_eq!(rev_int(-3005), -5003);
    }
}