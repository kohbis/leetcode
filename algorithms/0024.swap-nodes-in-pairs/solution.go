/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func swapPairs(head *ListNode) *ListNode {
	if head != nil && head.Next != nil {
		tmp := head.Next.Next
		head, head.Next.Next = head.Next, head
		head.Next.Next = swapPairs(tmp)
	}

	return head
}
