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
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut seq_left: Vec<i32> = vec![];
        let mut seq_right: Vec<i32> = vec![];

        Solution::leaf_sequence(root1.as_ref(), &mut seq_left);
        Solution::leaf_sequence(root2.as_ref(), &mut seq_right);

        seq_left == seq_right
    }

    fn leaf_sequence(root: Option<&Rc<RefCell<TreeNode>>>, current: &mut Vec<i32>) {
        if let Some(node) = root {
            let node = node.borrow();

            if node.left.is_none() && node.right.is_none() {
                current.push(node.val);
            }

            Solution::leaf_sequence(node.left.as_ref(), current);
            Solution::leaf_sequence(node.right.as_ref(), current);
        }
    }
}
