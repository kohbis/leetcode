func numberOfMatches(n int) int {
	count := 0

	for n > 1 {
		if n%2 == 0 {
			n = n / 2
			count += n
		} else {
			n = (n - 1) / 2
			count += n + 1
		}
	}

	return count
}
