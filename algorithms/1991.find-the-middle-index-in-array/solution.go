func findMiddleIndex(nums []int) int {
	sum := 0
	for _, n := range nums {
		sum += n
	}

	left, right := 0, sum
	for i := 0; i < len(nums); i++ {
		right -= nums[i]

		if left == right {
			return i
		}

		left += nums[i]
	}

	return -1
}
