func maxDistance(colors []int) int {
	l := len(colors)

	left := 0
	for colors[left] == colors[l-1] {
		left++
	}

	right := l - 1
	for colors[0] == colors[right] {
		right--
	}

	if (l - 1 - left) < right {
		return right
	} else {
		return l - 1 - left
	}
}
