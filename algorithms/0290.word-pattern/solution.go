func wordPattern(pattern string, s string) bool {
	m := map[rune]string{}
	words := strings.Fields(s)

	if len(words) != len(pattern) {
		return false
	}

	for i, c := range pattern {
		if _, ok := m[c]; ok {
			if m[c] != words[i] {
				return false
			}
		} else {
			m[c] = words[i]
		}
	}

	values := make(map[string]bool, len(pattern))
	for _, val := range m {
		if values[val] {
			return false
		} else {
			values[val] = true
		}
	}

	return true
}
