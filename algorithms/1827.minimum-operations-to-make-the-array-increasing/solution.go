func minOperations(nums []int) int {
	count := 0
	prev := 0
	for _, num := range nums {
		if prev >= num {
			count += (prev + 1) - num
			prev++
		} else {
			prev = num
		}
	}
	return count
}
