func isThree(n int) bool {
	divs := 2

	for i := 2; i < n; i++ {
		if n%i == 0 {
			divs++
		}

		if divs > 3 {
			return false
		}
	}

	return divs == 3
}
