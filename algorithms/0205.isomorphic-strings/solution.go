func isIsomorphic(s string, t string) bool {
	if len(s) != len(t) {
		return false
	}

	m := map[rune]rune{}
	t_chars := []rune(t)

	for i, c := range []rune(s) {
		if _, ok := m[c]; ok {
			if m[c] != t_chars[i] {
				return false
			}
		} else {
			m[c] = t_chars[i]
		}
	}

	values := make(map[rune]bool, len(s))
	for _, val := range m {
		if values[val] {
			return false
		} else {
			values[val] = true
		}
	}

	return true
}
