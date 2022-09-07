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
    pub fn tree2str(root: Node) -> String {
        let mut res = String::new();
        Solution::helper(&root, &mut res);
        res
    }

    fn helper(root: &Node, s: &mut String) {
        if let Some(node) = root {
            let node = node.borrow();
            *s += &node.val.to_string();

            match (&node.left.is_some(), &node.right.is_some()) {
                (true, true) => {
                    *s += "(";
                    Solution::helper(&node.left, s);
                    *s += ")(";
                    Solution::helper(&node.right, s);
                    *s += ")";
                }
                (true, false) => {
                    *s += "(";
                    Solution::helper(&node.left, s);
                    *s += ")";
                }
                (false, true) => {
                    *s += "()(";
                    Solution::helper(&node.right, s);
                    *s += ")";
                }
                _ => {}
            }
        }
    }
}
