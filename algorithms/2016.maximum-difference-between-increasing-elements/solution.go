func maximumDifference(nums []int) int {
	max := 0

	for i := 0; i < len(nums)-1; i++ {
		for j := i + 1; j < len(nums); j++ {
			if nums[j]-nums[i] > max {
				max = nums[j] - nums[i]
			}
		}
	}

	if max > 0 {
		return max
	} else {
		return -1
	}
}
