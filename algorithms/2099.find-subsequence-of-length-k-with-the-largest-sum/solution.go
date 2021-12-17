func maxSubsequence(nums []int, k int) []int {
	res := []int{}

	target := make([]int, len(nums))
	copy(target, nums)
	sort.Ints(target)
	target = target[len(nums)-k:]

	for _, n := range nums {
		idx := indexOf(n, target)
		if idx >= 0 {
			res = append(res, n)
			target = append(target[:idx], target[idx+1:]...)
		}
	}

	return res
}

func indexOf(n int, slice []int) int {
	for i, v := range slice {
		if n == v {
			return i
		}
	}

	return -1
}
