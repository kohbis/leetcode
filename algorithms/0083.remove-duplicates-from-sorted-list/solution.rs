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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        let mut res = head;

        let mut p = res.as_mut().unwrap();
        while let Some(t) = p.next.as_mut() {
            if p.val == t.val {
                p.next = t.next.take()
            } else {
                p = p.next.as_mut().unwrap()
            }
        }

        res
    }
}
