func minimumDifference(nums []int, k int) int {
	sort.Ints(nums)

	res := math.MaxInt32
	for i := k - 1; i < len(nums); i++ {
		diff := nums[i] - nums[i-k+1]
		if diff < res {
			res = diff
		}
	}

	return res
}
