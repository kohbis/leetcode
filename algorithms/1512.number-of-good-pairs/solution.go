func numIdenticalPairs(nums []int) int {
	res := 0
	for i := 0; i < len(nums); i++ {
		for j := i; j < len(nums); j++ {
			if nums[i] == nums[j] && i < j {
				res += 1
			}
		}
	}
	return res
}
