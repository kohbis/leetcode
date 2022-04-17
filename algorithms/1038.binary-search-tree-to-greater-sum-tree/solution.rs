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
    pub fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut sum = 0i32;
        Solution::helper(root.as_ref(), &mut sum);

        root
    }

    fn helper(root: Option<&Rc<RefCell<TreeNode>>>, sum: &mut i32) {
        if let Some(node) = root {
            let mut node = node.borrow_mut();

            Solution::helper(node.right.as_ref(), sum);

            node.val += *sum;
            *sum = node.val;

            Solution::helper(node.left.as_ref(), sum);
        }
    }
}
