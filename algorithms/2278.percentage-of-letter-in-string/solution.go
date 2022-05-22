func percentageLetter(s string, letter byte) int {
	m := map[byte]int{}
	for _, b := range []byte(s) {
		if _, ok := m[b]; ok {
			m[b] += 1
		} else {
			m[b] = 1
		}
	}

	if _, ok := m[letter]; ok {
		return 100 * m[letter] / len(s)
	}

	return 0
}

