func maxLengthBetweenEqualCharacters(s string) int {
	longest := -1
	m := map[rune]int{}

	for i, r := range s {
		if _, ok := m[r]; ok {
			diff := i - m[r] - 1
			if longest < diff {
				longest = diff
			}
		} else {
			m[r] = i
		}
	}

	return longest
}
