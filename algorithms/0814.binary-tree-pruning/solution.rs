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
    pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let node = root?;

        let left = node.borrow().left.clone();
        let right = node.borrow().right.clone();

        node.borrow_mut().left = Self::prune_tree(left);
        node.borrow_mut().right = Self::prune_tree(right);

        let left = node.borrow().left.clone();
        let right = node.borrow().right.clone();

        let val = node.borrow().val;
        if val == 0 && left.is_none() && right.is_none() {
            return None;
        }

        Some(node)
    }
}
