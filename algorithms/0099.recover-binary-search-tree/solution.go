/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func recoverTree(root *TreeNode) {
	vals := []int{}
	nodeVals(root, &vals)
	sort.Ints(vals)

	fmt.Println(vals[0])
	recover(root, &vals)
}

func nodeVals(node *TreeNode, vals *[]int) {
	if node == nil {
		return
	}

	*vals = append(*vals, node.Val)
	nodeVals(node.Left, vals)
	nodeVals(node.Right, vals)
}

func recover(node *TreeNode, vals *[]int) {
	if node == nil {
		return
	}

	recover(node.Left, vals)

	node.Val = (*vals)[0]
	*vals = (*vals)[1:]

	recover(node.Right, vals)
}
