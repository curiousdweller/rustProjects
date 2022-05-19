pub fn longest(s: String) -> Vec<char> {
    let str: Vec<char> = s.chars().collect();
    let mut max_substring = 0;
    let mut res = vec!['a'];
    for i in 0..str.len() {
        for j in i..str.len() {
            let sub = &str[i..=j];
            let backwards: Vec<&char> = sub.iter().rev().collect();
            let mut flag = 0;
            for (index, char) in sub.iter().enumerate() {
                if char == backwards[index] {
                    flag = 1;
                } else {
                    flag = 0;
                }
            }
            if flag == 1 {
                println!("{:?}", sub);
                if sub.len() > max_substring {
                    res = sub.to_vec();
                    max_substring = sub.len();
                }
            }
            // println!("{:?}", sub);
            // println!("Backwards: {:?}", backwards);
        }
    }
    res
}
