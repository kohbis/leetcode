func countTriples(n int) int {
	count := 0

	for a := 1; a <= n; a++ {
		for b := a + 1; b <= n; b++ {
			for c := b + 1; c <= n; c++ {
				if (a*a + b*b) == (c * c) {
					count += 2
					break
				}
			}
		}
	}

	return count
}

