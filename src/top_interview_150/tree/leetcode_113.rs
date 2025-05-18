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
use std::rc::Rc;
use std::cell::RefCell;

struct Solution;
impl Solution {
    pub fn dfs(
        root: Option<Rc<RefCell<TreeNode>>>,
        target_sum: i32, path: &mut Vec<i32>, sum: &mut i32, ans: &mut Vec<Vec<i32>>) {
        if let Some(node) = root {
            let node = node.borrow();
            *sum += node.val;
            path.push(node.val);
            if node.left.is_none() && node.right.is_none() && *sum == target_sum {
                ans.push(path.clone());
            }
            Self::dfs(node.left.clone(), target_sum, path, sum, ans);
            Self::dfs(node.right.clone(), target_sum, path, sum, ans);
            path.pop();
            *sum -= node.val;
        }
    }
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        } else {
            let mut ans = vec![];
            let mut path = vec![];
            let mut sum = 0;
            Self::dfs(root, target_sum, &mut path, &mut sum, &mut ans);
            return ans;
        }
    }
}


// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::rc::Rc;
//     use std::cell::RefCell;

//     #[test]
//     fn test_path_sum() {
//     }
// }


fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    let left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    let right = Some(Rc::new(RefCell::new(TreeNode::new(8))));
    root.as_ref().unwrap().borrow_mut().left = left;
    root.as_ref().unwrap().borrow_mut().right = right;
    let target_sum = 9;
    let result = Solution::path_sum(root, target_sum);
    println!("{:?}", result);
}