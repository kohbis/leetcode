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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut balanced = true;
        Solution::dfs(&root, 0, &mut balanced);
        balanced
    }

    fn dfs(root: &Node, hight: i32, balanced: &mut bool) -> i32 {
        match root {
            Some(node) => {
                let node = node.borrow();

                let left = Solution::dfs(&node.left, hight + 1, balanced);
                let right = Solution::dfs(&node.right, hight + 1, balanced);

                if (left - right).abs() > 1 {
                    *balanced = false;
                }

                left.max(right)
            }
            _ => hight - 1,
        }
    }
}
