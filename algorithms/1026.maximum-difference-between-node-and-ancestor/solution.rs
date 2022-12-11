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
use std::cell::RefCell;
use std::cmp::{max, min};
use std::rc::Rc;
impl Solution {
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let curr_max = root.clone().unwrap().borrow().val;
        let curr_min = root.clone().unwrap().borrow().val;
        return Solution::max_diff(root.as_ref(), curr_max, curr_min);
    }

    fn max_diff(root: Option<&Rc<RefCell<TreeNode>>>, curr_max: i32, curr_min: i32) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();
            let curr_max = curr_max.max(node.val);
            let curr_min = curr_min.min(node.val);

            let left = Solution::max_diff(node.left.as_ref(), curr_max, curr_min);
            let right = Solution::max_diff(node.right.as_ref(), curr_max, curr_min);

            left.max(right)
        } else {
            curr_max - curr_min
        }
    }
}
