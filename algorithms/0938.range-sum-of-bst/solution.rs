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
use std::rc::Rc;
impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        if root.is_none() {
            return 0;
        }

        let mut res = 0;
        let root = root.as_ref().unwrap().borrow();
        if low <= root.val && root.val <= high {
            res += root.val;
        }
        res + Solution::range_sum_bst(root.left.clone(), low, high)
            + Solution::range_sum_bst(root.right.clone(), low, high)
    }
}
