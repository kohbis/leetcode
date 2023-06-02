// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut vals: Vec<i32> = vec![];
        let mut head = head;
        while let Some(node) = head {
            vals.push(node.val);
            head = node.next;
        }

        let (mut left, mut right) = (0, vals.len() - 1);
        while left < right {
            if vals[left] != vals[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }
}
