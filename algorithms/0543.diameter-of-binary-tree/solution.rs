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
    pub fn diameter_of_binary_tree(root: Node) -> i32 {
        let mut longest = 0;
        Self::depth(root, &mut longest);
        longest
    }

    fn depth(node: Node, longest: &mut i32) -> i32 {
        if let Some(node) = node {
            let left = Self::depth(node.borrow().left.clone(), longest);
            let right = Self::depth(node.borrow().right.clone(), longest);
            *longest = std::cmp::max(*longest, left + right);
            1 + left.max(right)
        } else {
            0
        }
    }
}
