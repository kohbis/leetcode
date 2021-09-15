func countQuadruplets(nums []int) int {
	l := len(nums)
	count := 0

	for a := 0; a < l-3; a++ {
		for b := a + 1; b < l-2; b++ {
			for c := b + 1; c < l-1; c++ {
				for d := c + 1; d < l; d++ {
					if nums[a]+nums[b]+nums[c] == nums[d] {
						count++
					}
				}
			}
		}
	}

	return count
}
