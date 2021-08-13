func maximumProduct(nums []int) int {
	sort.Slice(nums, func(i, j int) bool {
		return nums[i] < nums[j]
	})

	l := len(nums)
	a := nums[l-1] * nums[l-2] * nums[l-3]
	b := nums[0] * nums[1] * nums[l-1]

	if a > b {
		return a
	} else {
		return b
	}
}
