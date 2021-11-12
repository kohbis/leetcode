func minStartValue(nums []int) int {
	min := math.MaxInt32
	sum := 0

	for _, n := range nums {
		sum += n
		if sum < min {
			min = sum
		}
	}

	if 1 > min*-1+1 {
		return 1
	} else {
		return min*-1 + 1
	}
}
