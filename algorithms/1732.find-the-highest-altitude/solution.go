func largestAltitude(gain []int) int {
	max := 0
	pos := 0

	for _, g := range gain {
		pos += g
		if pos > max {
			max = pos
		}
	}

	return max
}
