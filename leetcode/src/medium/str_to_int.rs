// Begin at 16:30
pub fn str_to_int(s: String) -> i32 {
    let filtred = s.chars().filter(|char| {
        char.is_numeric()
    }).collect::<Vec<char>>();
    // SImple algo to check if the entire num is contained within the string
    let s_as_vec = s.chars().collect::<Vec<char>>();
    let mut j = 0;
    let mut is_negative = false;
    // find the starting point where filtred is the same as original
    // check for negative here aswell.
    while filtred[0] != s_as_vec[j] {
            j += 1;
        }

        if j > 0 && s_as_vec[j - 1] == '-' {
            is_negative = true;
        }
    let mut cutoff = filtred.len() - 1;
    for i in 0..filtred.len() {
        if filtred[i] == s_as_vec[j] {
            j += 1;
            continue;
        }
        else {
            cutoff = i - 1;
            break
        }
    }
    let fully_processed = filtred[0..=cutoff].to_owned().iter().collect::<String>();

    println!("{:?}", fully_processed);
    let final_string = fully_processed.parse::<i32>();

    match final_string {
        Ok(res) => if is_negative {
            res * -1
        } else {
            res
        },
        Err(_) => if is_negative {
            i32::min_value()
        } else {
            i32::max_value()
        }
    }

}

pub fn my_atoi(str: String) -> i32 {
    let (n, s) = match str.chars().skip_while(|x| x.is_whitespace()).take(1).next() {
        Some('+') => (1, 1),
        Some(x) if x.is_digit(10) => (0, 1),
        Some('-') => (1, -1),
        _ => return 0,
    };
    let mut res = 0i32;
    let overflow = if s > 0 { std::i32::MAX } else { std::i32::MIN };
    for c in str.chars().skip_while(|x| x.is_whitespace()).skip(n)
        .take_while(|x| x.is_digit(10)) {
            let (r, o) = res.overflowing_mul(10);
            if o { return overflow; }
            let (r, o) = r.overflowing_add(s*(c as i32 - '0' as i32));
            if o { return overflow; }
            res = r;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ex_1() {
        assert_eq!(my_atoi("42".to_owned()), 42);
    }
    #[test]
    fn ex_2() {
        assert_eq!(my_atoi("-42".to_owned()), -42);
    }
    #[test]
    fn ex_3() {
        assert_eq!(my_atoi("4193 with words".to_owned()), 4193);
    }
    #[test]
    fn ex_4() {
        assert_eq!(my_atoi("multiple numbers 9 4193 with words".to_owned()), 9);
    }

}