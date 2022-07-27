use std::collections::HashSet;
pub fn naive_remove_duplicates(s: String, k: i32) -> String {
    let mut dup = 'a';
    let mut count = 0;
    let mut to_delete = HashSet::new();
    let mut unchanged = false;
    let mut temp_vec = Vec::with_capacity(k as usize);

    while unchanged == false {
        unchanged = true;
        let mut flag = true;
        for (index,char) in s.char_indices() {

            if to_delete.get(&index).is_some() {

                continue;

            } else if flag {
                temp_vec.clear();
                count = 0;
                dup = char;
                println!("FIRST INDEXXXXX: {}", index);
                flag = false;
            }
            // println!("CHAR:  {}  DUP:   {}  INDEX:  {} ", char, dup, index);
            if char == dup {
                count += 1;
                temp_vec.push(index);
                
            } else {
                count = 1;
                temp_vec.clear();
                temp_vec.push(index);
            }

            if count == k  {
                for num in temp_vec.clone() {
                    to_delete.insert(num);
                }
                unchanged = false;
                temp_vec.clear();
                temp_vec.push(index);
        
            }
            dup = char;

            println!("{:?}", temp_vec);
            
        }       
    }
    let s2 = s.char_indices().filter(|(index, char)| {
        to_delete.get(index).is_none()
    }).collect::<Vec<_>>();
    println!("{:?}",to_delete );
    println!("{:?}", s2);
    String::new()
}


pub fn stack_remove(s: String, k: i32) -> String {
    let mut res = String::new();
    let mut stack: Vec<(i32, char)> = Vec::new();
    for char in s.chars() {
        if stack.len() > 0 && stack[stack.len() - 1].1 == char {
            let len = stack.len();
            stack[len - 1].0 += 1;
        } else {
            stack.push((1, char));
        }

        if stack[stack.len() - 1].0 == k {
            let _ = stack.pop();
        }
    }

    for (_, char) in stack {
        res.push(char);
    }

    println!("{}", res);
    res
}

#[cfg(test)] 
mod tests {
    use super::*; 
    #[test]
    fn rem_adj_dup_ll_test() {
        stack_remove(String::from("pbbcggttciiippooaais"), 2);
    }
}