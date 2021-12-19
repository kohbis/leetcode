/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func addTwoNumbers(l1 *ListNode, l2 *ListNode) *ListNode {
	val := l1.Val + l2.Val
	res := &ListNode{Val: val % 10, Next: nil}
	current := res

	over := 0
	if val >= 10 {
		over = 1
	}

	l1 = l1.Next
	l2 = l2.Next

	for over > 0 || l1 != nil || l2 != nil {
		val = over
		if l1 != nil {
			val += l1.Val
		}
		if l2 != nil {
			val += l2.Val
		}

		current.Next = &ListNode{Val: val % 10, Next: nil}

		over = 0
		if val >= 10 {
			over = 1
		}

		if l1 != nil {
			l1 = l1.Next
		}
		if l2 != nil {
			l2 = l2.Next
		}

		current = current.Next
	}

	return res
}
