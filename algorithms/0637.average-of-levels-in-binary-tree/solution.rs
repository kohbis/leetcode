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
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut res: Vec<f64> = vec![];
        let mut nodes = VecDeque::new();

        if let Some(node) = root {
            nodes.push_back(node);
        }

        while nodes.len() > 0 {
            let len = nodes.len();
            let mut sum = 0.0f64;

            for _ in 0..len {
                if let Some(node) = nodes.pop_front() {
                    sum += node.borrow().val as f64;

                    if let Some(left) = node.borrow_mut().left.take() {
                        nodes.push_back(left);
                    }
                    if let Some(right) = node.borrow_mut().right.take() {
                        nodes.push_back(right);
                    }
                }
            }

            res.push(sum / len as f64)
        }

        res
    }
}
