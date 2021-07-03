func canBeIncreasing(nums []int) bool {
	pre := nums[0]
	removed := false

	for i := 1; i < len(nums); i++ {
		if pre < nums[i] {
			pre = nums[i]
		} else {
			if removed {
				return false
			}
			removed = true

			if i == 1 || nums[i-2] < nums[i] {
				pre = nums[i]
			}
		}
	}

	return true
}
