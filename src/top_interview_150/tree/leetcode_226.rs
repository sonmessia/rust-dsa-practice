use std::rc::Rc;
use std::cell::RefCell;
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
struct Solution;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) ->  Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let mut node_borrowed = node.borrow_mut();
            let left = node_borrowed.left.take();
            let right = node_borrowed.right.take();
            node_borrowed.left = Self::invert_tree(right);
            node_borrowed.right = Self::invert_tree(left);
            Some(node)
        } else {
            None
        }
        
    }
}