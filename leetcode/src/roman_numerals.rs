use std::collections::HashMap;
pub fn translate(s: String) -> i32 {
    let mut roman_numerals: HashMap<char, i32> = HashMap::with_capacity(8);
    roman_numerals.insert('X', 10);
    roman_numerals.insert('I', 1);
    roman_numerals.insert('L', 50);
    roman_numerals.insert('C', 100);
    roman_numerals.insert('D', 500);
    roman_numerals.insert('M', 1000);
    roman_numerals.insert('V', 5);
    roman_numerals.insert('Z', 0);

    let (mut res, mut prev) = (0, 'Z');
    s.chars().rev().for_each(|c| {
        let c_num = roman_numerals.get(&c).unwrap();
        let prev_num = roman_numerals.get(&prev).unwrap();
        if c_num < prev_num {
            res -= c_num;
        } else {
            res += c_num;
            println!("Else Statement: {}", res);
        }
        prev = c;
    });

    res
    // let  mapping = s.chars().map(|a| {
    //     *roman_numerals.get(&a).unwrap()
    // });

    // // We want to Keep track of previous and current and then compare to some combinations
    // mapping.sum()
}
