func countKDifference(nums []int, k int) int {
	count := 0

	for i := 0; i < len(nums)-1; i++ {
		for j := i + 1; j < len(nums); j++ {
			if int(math.Abs(float64(nums[i])-float64(nums[j]))) == k {
				count++
			}
		}
	}

	return count
}
