func check(nums []int) bool {
	l := len(nums)
	rotate := false
	max := 0

	for i, _ := range nums {
		var left int
		if i == 0 {
			left = nums[l-1]
		} else {
			left = nums[i-1]
		}

		if nums[i] >= left {
			if rotate && nums[i] > max {
				return false
			}
		} else {
			if rotate {
				return false
			}
			rotate = true
			max = left
		}
	}

	return true
}
