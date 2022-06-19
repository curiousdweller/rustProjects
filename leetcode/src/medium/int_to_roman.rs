// Num in range of 1 <= n <= 3999

// We want to identify the unit in decimal by examning the length of the int
// 34 --> 2 --> tens
// 3545 --> thousands
// anything more than 4 digits is defaulted to thousands
use std::collections::HashMap;
pub fn int_to_roman(num: i32) -> String {
        let mut res = vec![];
        let s = num.to_string();
        let mut sub_unit = s.len() as i32;
        let mut unit_hashmap = HashMap::new(); 
        unit_hashmap.insert((1, true), 'I');
        unit_hashmap.insert((1, false), 'V');
        unit_hashmap.insert((2, true), 'X');
        unit_hashmap.insert((2, false), 'L');
        unit_hashmap.insert((3, true), 'C');
        unit_hashmap.insert((3, false), 'D');
        unit_hashmap.insert((4, true), 'M');

        for char in s.chars() {
            let mut temp_res: String;
            if char < '5' && char != '9' && char != '4' || (char < '5' && sub_unit == 4){
                let roman_char = unit_hashmap.get(&(sub_unit, true)).unwrap();
                let num: i32 = char.to_string().parse().unwrap();
                let mut temp_vec = vec![]; 
                for i in 0..num {
                    temp_vec.push(roman_char.to_string());
                }
                 temp_res = temp_vec.concat();
            } else if (char != '9' && char != '4') || sub_unit == 4 {
                eprint!("\n ********Sub Unit:{} ", sub_unit);
                let roman_num = unit_hashmap.get(&(sub_unit, false)).unwrap();
                let num: i32 = char.to_string().parse().unwrap();
                let mut temp_vec = vec![roman_num.to_string()];
                for i in 5..num{
                    temp_vec.push(unit_hashmap.get(&(sub_unit, true)).unwrap().to_string());
                }

                temp_res = temp_vec.concat();
            } else {
                eprint!("Sub_UNit: {}", sub_unit);
                if char == '4' {
                     temp_res = format!("{}{}", unit_hashmap.get(&(sub_unit, true)).unwrap().to_string(), unit_hashmap.get(&(sub_unit, false)).unwrap()).to_string();
                } else {
                      temp_res = format!("{}{}", unit_hashmap.get(&(sub_unit, true)).unwrap().to_string(), unit_hashmap.get(&(sub_unit + 1, true)).unwrap().to_string());
                }
            }
            res.push(temp_res);
            sub_unit -= 1;
        }

        res.concat()



}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn roman_test_1() {
        assert_eq!(int_to_roman(3), String::from("III"));
    }
    #[test]
    fn roman_test_2() {
        assert_eq!(int_to_roman(58), String::from("LVIII"));
    }
    #[test]
    fn roman_test_3() {
        assert_eq!(int_to_roman(1994), String::from("MCMXCIV"));
    }
}