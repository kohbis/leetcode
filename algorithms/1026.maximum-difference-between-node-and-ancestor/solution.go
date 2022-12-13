/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func maxAncestorDiff(root *TreeNode) int {
	return maxDiff(root, root.Val, root.Val)
}

func maxDiff(node *TreeNode, max int, min int) int {
	if node == nil {
		return max - min
	}

	if node.Val > max {
		max = node.Val
	}
	if node.Val < min {
		min = node.Val
	}

	left := maxDiff(node.Left, max, min)
	right := maxDiff(node.Right, max, min)

	if left > right {
		return left
	} else {
		return right
	}
}

