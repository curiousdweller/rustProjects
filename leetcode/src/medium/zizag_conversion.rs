use std::vec;

fn make_zizag(string: String, n: usize) -> String {
    let mut downwards = true;
    let mut strings = vec![String::new(); n];

    let mut i = 0;
    for char in string.chars() {
        strings[i].push(char);
        if i == n - 1 {
            downwards = false;
        }
        if i == 0 {
            downwards = true;
        }
        if downwards {
            i += 1;
        } else {
            i -= 1;
        }
    }
    strings.concat()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn zizag_test() {
        assert_eq!(
            make_zizag(String::from("paypalishiring"), 3),
            String::from("pahnaplsiigyir")
        );
    }
}
