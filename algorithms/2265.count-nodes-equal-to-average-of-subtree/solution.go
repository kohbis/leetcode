/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func averageOfSubtree(root *TreeNode) int {
	result, _, _ := helper(root)
	return result
}

func helper(root *TreeNode) (result int, nodeCount int, nodeSum int) {
	if root == nil {
		return
	}

	resultL, countL, sumL := helper(root.Left)
	resultR, countR, sumR := helper(root.Right)

	result = resultL + resultR
	nodeCount = countL + countR + 1
	nodeSum = root.Val + sumL + sumR

	if root.Val == nodeSum/nodeCount {
		result++
	}

	return
}
