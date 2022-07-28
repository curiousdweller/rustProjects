// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
pub use std::rc::Rc;
pub use std::cell::RefCell;
pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = vec![];
        if root.is_none() {
            return res
        }

        let root = root.unwrap();

        res.append(&mut inorder_traversal(root.borrow_mut().left.take()));
        res.push(root.borrow().val);
        res.append(&mut inorder_traversal(root.borrow_mut().right.take()));
        res
}

pub fn iterative(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = vec![];
    let mut node = root;
    let mut stack: Vec<Rc<RefCell<TreeNode>>> = vec![];

    while node.is_some() || !stack.is_empty()  {
        println!("A COUPLE TIMES");
        while let Some(left_child) = node {
            stack.push(Rc::clone(&left_child));
            node = left_child.borrow_mut().left.take();
        }
        // Now at the top of our stack we have the 'deepest' node, check if this has 
        // any right children then pop and repeat
        if let Some(n) = stack.pop() {
            println!("{:?}", n);
            res.push(n.borrow().val);
            node = n.borrow_mut().right.take();
            println!("{:?}", node);
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn in_order_test() {
        let node = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                right: None,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None
                })))
                })))
        })));
        let node2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                right: None,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None
                })))
                })))
        })));
`
    
        assert_eq!(inorder_traversal(node), iterative(node2));
    }
}