func checkIfPangram(sentence string) bool {
	m := map[rune]int{}
	for _, r := range sentence {
		if _, ok := m[r]; ok {
			m[r] += 1
		} else {
			m[r] = 1
		}
	}
	return len(m) == 26
}
