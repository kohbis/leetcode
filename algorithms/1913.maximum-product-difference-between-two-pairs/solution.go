func maxProductDifference(nums []int) int {
	l := len(nums)
	sort.Slice(nums, func(i, j int) bool {
		return nums[i] < nums[j]
	})

	return nums[l-1]*nums[l-2] - nums[0]*nums[1]
}
