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
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match(root) {
            Some(node) => {
                let left = node.borrow().left.clone();
                let right = node.borrow().right.clone();
                Self::is_mirror(left, right)
            },
            None => true,
        }
    }

    fn is_mirror(a: Option<Rc<RefCell<TreeNode>>>, b: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match(a, b) {
            (Some(a), Some(b)) => {
                let a = a.borrow();
                let b = b.borrow();

                if a.val == b.val {
                    Self::is_mirror(a.left.clone(), b.right.clone())
                    && Self::is_mirror(b.left.clone(), a.right.clone())
                } else {
                    false
                }
            },
            (None, None) => true,
            _ => false,
        }
    }
}
