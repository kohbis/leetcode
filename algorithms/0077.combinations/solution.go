func combine(n int, k int) [][]int {
	res := make([][]int, 0)

	helper(n, k, 1, []int{}, &res)

	return res
}

func helper(n int, k int, index int, current []int, res *[][]int) {
	if k == len(current) {
		curr := make([]int, len(current))
		copy(curr, current)
		*res = append(*res, curr)
	} else {
		for i := index; i <= n; i++ {
			current = append(current, i)
			helper(n, k, i+1, current, res)
			current = current[:len(current)-1]
		}
	}
}
