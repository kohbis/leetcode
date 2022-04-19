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
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut vals: Vec<i32> = vec![];
        Solution::node_vals(root.as_ref(), &mut vals);
        vals.sort_unstable();
        vals.reverse();

        Solution::recover(root, &mut vals);
    }

    fn node_vals(root: Option<&Rc<RefCell<TreeNode>>>, vals: &mut Vec<i32>) {
        if let Some(node) = root {
            let mut node = node.borrow();

            Solution::node_vals(node.left.as_ref(), vals);
            Solution::node_vals(node.right.as_ref(), vals);

            vals.push(node.val);
        }
    }

    fn recover(root: &mut Option<Rc<RefCell<TreeNode>>>, vals: &mut Vec<i32>) {
        if let Some(node) = root {
            let mut node = node.borrow_mut();

            Solution::recover(&mut node.left, vals);

            if let Some(x) = vals.pop() {
                node.val = x;
            }

            Solution::recover(&mut node.right, vals);
        }
    }
}
