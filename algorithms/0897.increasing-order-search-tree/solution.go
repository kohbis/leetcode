/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func increasingBST(root *TreeNode) *TreeNode {
	values := []int{}
	helper(root, &values)

	res := &TreeNode{Val: values[0], Left: nil, Right: nil}
	tmp := res
	for i := 1; i < len(values); i++ {
		tmp.Right = &TreeNode{Val: values[i], Left: nil, Right: nil}
		tmp = tmp.Right
	}

	return res
}

func helper(root *TreeNode, values *[]int) {
	if root == nil {
		return
	}

	helper(root.Left, values)
	*values = append(*values, root.Val)
	helper(root.Right, values)
}

