use std::rc::Rc;
use std::cell::RefCell;
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

struct Solution;

impl Solution {
    pub fn max_deep(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            cmp::max (
                Self::max_deep(node.borrow().left.clone()),
                Self::max_deep(node.borrow().right.clone())
            ) + 1
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn test_max_deep() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        root.as_ref().unwrap().borrow_mut().left = left;
        root.as_ref().unwrap().borrow_mut().right = right;
        assert_eq!(Solution::max_deep(root), 2);
    }
}