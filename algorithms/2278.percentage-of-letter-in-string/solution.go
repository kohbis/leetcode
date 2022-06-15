func percentageLetter(s string, letter byte) int {
	count := 0
	for _, b := range []byte(s) {
		if b == letter {
			count++
		}
	}
	return 100 * count / len(s)
}


