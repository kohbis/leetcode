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
    pub fn lowest_common_ancestor(root: Node, p: Node, q: Node) -> Node {
        let mut root = root;
        match (p, q) {
            (Some(a), Some(b)) => {
                let a = a.borrow();
                let b = b.borrow();

                while let Some(node) = root.clone() {
                    let node = node.borrow();

                    if node.val > a.val && node.val > b.val {
                        root = node.left.clone();
                    } else if node.val < a.val && node.val < b.val {
                        root = node.right.clone();
                    } else {
                        return root;
                    }
                }

                None
            }
            _ => None,
        }
    }
}
