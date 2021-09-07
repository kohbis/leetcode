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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut res: Option<Box<ListNode>> = None;
        let mut curr = head;

        while let Some(mut node) = curr {
            let mut next = node.next.take();
            node.next = res.take();
            res = Some(node);
            curr = next.take();
        }

        res
    }
}
