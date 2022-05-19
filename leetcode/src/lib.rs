pub mod is_palindrome;
mod lists;
pub mod longest_palindrome;
pub mod medium;
mod roman_numerals;
pub mod sleep_sort;
pub mod twosum;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ListNode {
    num: u32,
    pointer: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(num: u32) -> Self {
        ListNode {
            num,
            pointer: Some(Box::new(ListNode {
                num: 5,
                pointer: None,
            })),
        }
    }
    // Consumes list and returns new longer list
    pub fn push(mut self, num: u32) -> Self {
        // variable list takes ownership of self.pointer unless it implements Copy by default. Use ref!
        while let Some(ref mut list) = self.pointer {
            println!("hello");
            match list.pointer {
                Some(_) => continue,
                None => {
                    list.pointer = Some(Box::new(ListNode { num, pointer: None }));
                }
            }
        }

        self.pointer = Some(Box::new(ListNode { num, pointer: None }));

        self
    }
}
