// Definition for singly-linked list.
use std::mem;
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Vec<i32> {
    //  Update l1 so that it becomes next (field in ListNode)
    let (mut l1, mut l2) = (l1, l2);
    let mut res_list = ListNode::new(0);
    let mut res = vec![];
    let mut carry = false;
    let mut first = 0;
    let mut second = 0;

    loop {
        let next_l1 = match l1 {
            Some(node) => {
                first = node.val;
                node.next
            }
            None => break,
        };

        let next_l2 = match l2 {
            Some(node) => {
                second = node.val;
                node.next
            }
            None => break,
        };

        let mut temp = 0;

        if carry {
            temp += 1;
        }

        temp += carry_check(first, second, &mut carry);
        let new_node = Some(Box::new(ListNode {
            val: temp,
            next: None,
        }));

        res.push(temp);
        l1 = next_l1;
        l2 = next_l2;
    }

    res
}

fn carry_check(n: i32, m: i32, carry: &mut bool) -> i32 {
    if (n + m) >= 10 {
        *carry = true;
        (n + m) % 10
    } else {
        *carry = false;
        n + m
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_adder() {
        let l1 = ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        };
        let l2 = ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode { val: 2, next: None })),
            })),
        };

        println!(
            "{:?}",
            add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2)))
        );
    }
}
