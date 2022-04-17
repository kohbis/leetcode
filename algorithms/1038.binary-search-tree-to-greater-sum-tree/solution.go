/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func bstToGst(root *TreeNode) *TreeNode {
	sum := 0
	helper(root, &sum)

	return root
}

func helper(node *TreeNode, sum *int) {
	if node == nil {
		return
	}

	helper(node.Right, sum)

	node.Val += *sum
	*sum = node.Val

	helper(node.Left, sum)
}
