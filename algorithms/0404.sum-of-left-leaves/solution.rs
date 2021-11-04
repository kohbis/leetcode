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

type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn sum_of_left_leaves(root: Node) -> i32 {
        fn helper(root: &Node, is_left: bool) -> i32 {
            match root {
                Some(node) => {
                    let node = node.borrow();
                    match (&node.left, &node.right) {
                        (None, None) => {
                            if is_left {
                                return node.val;
                            }
                        }
                        (l, r) => {
                            return helper(l, true) + helper(r, false);
                        }
                    }

                    0
                }
                None => 0,
            }
        }

        helper(&root, false)
    }
}
