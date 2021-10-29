func minimumMoves(s string) int {
	runes := []rune(s)

	count := 0
	i := 0

	for len(s) > i {
		if runes[i] == 'X' {
			count += 1
			i += 2
		}

		i += 1
	}

	return count
}
