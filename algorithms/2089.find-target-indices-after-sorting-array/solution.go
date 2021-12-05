func targetIndices(nums []int, target int) []int {
	var targetCount int
	var smallerCount int

	for _, n := range nums {
		if target == n {
			targetCount++
		} else if target > n {
			smallerCount++
		}
	}

	var res []int
	for i := smallerCount; i < smallerCount+targetCount; i++ {
		res = append(res, i)
	}

	return res
}
