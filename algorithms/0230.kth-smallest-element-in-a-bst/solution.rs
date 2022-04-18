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
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut vals: Vec<i32> = vec![];
        Solution::helper(root.as_ref(), &mut vals);

        if vals.len() > 0 {
            vals.sort_unstable();
            vals[k as usize - 1]
        } else {
            -1 // unreachable!()
        }
    }

    fn helper(root: Option<&Rc<RefCell<TreeNode>>>, vals: &mut Vec<i32>) {
        if let Some(node) = root {
            let mut node = node.borrow_mut();

            Solution::helper(node.left.as_ref(), vals);
            Solution::helper(node.right.as_ref(), vals);

            vals.push(node.val);
        }
    }
}
