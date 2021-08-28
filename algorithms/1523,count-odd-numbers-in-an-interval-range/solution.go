func countOdds(low int, high int) int {
	sum := (high - low) / 2

	if (high-low)%2 == 1 {
		sum += 1
	} else if low%2 == 1 {
		sum += 1
	}

	return sum
}
