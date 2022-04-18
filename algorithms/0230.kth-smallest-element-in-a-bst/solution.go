/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func kthSmallest(root *TreeNode, k int) int {
	vals := []int{}
	helper(root, &vals)

	if len(vals) > 0 {
		sort.Ints(vals)
		return vals[k-1]
	} else {
		return -1 // unreachable
	}
}

func helper(node *TreeNode, vals *[]int) {
	if node == nil {
		return
	}

	helper(node.Left, vals)
	helper(node.Right, vals)

	*vals = append(*vals, node.Val)
}
