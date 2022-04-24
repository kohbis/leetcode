/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func constructMaximumBinaryTree(nums []int) *TreeNode {
	switch len(nums) {
	case 0:
		return nil
	case 1:
		return &TreeNode{nums[0], nil, nil}
	default:
		max, maxIndex := getMaxWithIndex(nums)
		left, right := nums[:maxIndex], nums[maxIndex+1:]
		return &TreeNode{max, constructMaximumBinaryTree(left), constructMaximumBinaryTree(right)}
	}
}

func getMaxWithIndex(nums []int) (int, int) {
	max, maxIndex := -1, -1
	for i, v := range nums {
		if v > max {
			max, maxIndex = v, i
		}
	}
	return max, maxIndex
}
