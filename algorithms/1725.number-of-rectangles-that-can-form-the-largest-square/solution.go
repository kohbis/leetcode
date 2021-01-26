func countGoodRectangles(rectangles [][]int) int {
	maxLen := 0
	maxLenCount := 0

	for _, r := range rectangles {
		minLen := min(r[0], r[1])

		if minLen > maxLen {
			maxLen = minLen
			maxLenCount = 1
		} else if minLen == maxLen {
			maxLenCount += 1
		}
	}

	return maxLenCount
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}
