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
    pub fn find_bottom_left_value(root: Node) -> i32 {
        let mut leftmost = 0;
        let mut max_depth = 0;
        Self::dfs(root, &mut leftmost, &mut max_depth, 1);
        leftmost
    }

    fn dfs(node: Node, leftmost: &mut i32, max_depth: &mut i32, depth: i32) {
        if let Some(node) = node {
            if depth > *max_depth {
                *max_depth = depth;
                *leftmost = node.borrow().val;
            }
            Self::dfs(node.borrow().left.clone(), leftmost, max_depth, depth + 1);
            Self::dfs(node.borrow().right.clone(), leftmost, max_depth, depth + 1);
        }
    }
}
