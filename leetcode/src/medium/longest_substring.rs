use std::collections::HashMap;

pub fn longest_substring(s: String) -> usize {
    let s = s.chars().collect::<Vec<char>>();
    let mut max: usize = 0;

    for i in 0..s.len() {
        for j in i..s.len() {
            let sub = &s[i..=j];
            if not_repeating(sub) && sub.len() > max {
                max = sub.len();
            }
        }
    }

    max
}

pub fn length_of_longest_substring(s: String) -> i32 {
    use std::collections::HashMap;

    let len = s.len();
    let mut max_len = 0_i32;
    let mut seen = HashMap::new();
    let mut start = 0_usize;

    for (idx, c) in s.char_indices() {
        match seen.insert(c, idx) {
            Some(existing_idx) if existing_idx >= start => {
                println!("{}", start as i32 - existing_idx as i32);
                println!("MAX LEN:{}", max_len);
                start = existing_idx + 1;
            }
            _ => {
                max_len = std::cmp::max(max_len, idx as i32 - start as i32 + 1);
            }
        }
    }

    max_len
}

fn not_repeating(s: &[char]) -> bool {
    let mut hash_map = HashMap::new();

    for i in s {
        if hash_map.get(i).is_some() {
            return false;
        }
        hash_map.insert(i, true);
    }

    true
}
