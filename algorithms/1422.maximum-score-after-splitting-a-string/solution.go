func maxScore(s string) int {
	max := 0
	size := len(s)
	scores := make([]int, size-1)

	countZero := 0
	countOne := 0

	for left := 0; left < size-1; left++ {
		if s[left] == '0' {
			countZero += 1
		}
		scores[left] += countZero
	}

	for right := size - 1; right > 0; right-- {
		if s[right] == '1' {
			countOne += 1
		}
		scores[right-1] += countOne

		if scores[right-1] > max {
			max = scores[right-1]
		}
	}

	return max
}
