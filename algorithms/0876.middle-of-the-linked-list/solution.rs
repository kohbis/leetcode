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
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut len = 0usize;
        let mut tmp = head.as_ref();
        while let Some(node) = tmp.as_ref() {
            len += 1;
            tmp = node.next.as_ref();
        }

        let mut head = head;
        for _ in 0..len / 2 {
            if let Some(node) = head {
                head = node.next;
            }
        }

        head
    }
}
