func maxArea(height []int) int {
	max := 0
	left, right := 0, len(height)-1

	for left < right {
		amount := min(height[left], height[right]) * (right - left)

		if max < amount {
			max = amount
		}

		if height[left] > height[right] {
			right--
		} else {
			left++
		}
	}

	return max
}

func min(i int, j int) int {
	if i < j {
		return i
	}
	return j
}
