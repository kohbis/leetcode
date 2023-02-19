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
use std::collections::BTreeMap;

type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn zigzag_level_order(root: Node) -> Vec<Vec<i32>> {
        let mut zigzag: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
        Solution::helper(&root, 0, &mut zigzag);
        let mut res: Vec<Vec<i32>> = vec![];
        for (k, mut v) in zigzag {
            if k % 2 == 1 {
                v.reverse();
            }
            res.push(v);
        }
        res
    }

    fn helper(root: &Node, level: i32, zigzag: &mut BTreeMap<i32, Vec<i32>>) {
        if let Some(node) = root {
            let node = node.borrow();
            let entry = zigzag.entry(level).or_insert(vec![]);
            (*entry).push(node.val);
            Solution::helper(&node.left, level + 1, zigzag);
            Solution::helper(&node.right, level + 1, zigzag);
        }
    }
}
