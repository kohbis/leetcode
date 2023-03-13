/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func isSymmetric(root *TreeNode) bool {
	return root == nil || is_mirror(root.Left, root.Right)
}

func is_mirror(a *TreeNode, b *TreeNode) bool {
	if a == nil || b == nil {
		return a == nil && b == nil
	}

	if a.Val == b.Val {
		return is_mirror(a.Left, b.Right) && is_mirror(b.Left, a.Right)
	} else {
		return false
	}
}
