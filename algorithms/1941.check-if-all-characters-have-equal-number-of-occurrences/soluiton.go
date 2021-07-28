func areOccurrencesEqual(s string) bool {
	counts := map[rune]int{}

	for _, r := range s {
		if _, ok := counts[r]; ok {
			counts[r] += 1
		} else {
			counts[r] = 1
		}
	}

	n := 0
	for _, v := range counts {
		if n > 0 {
			if n != v {
				return false
			}
		} else {
			n = v
		}
	}

	return true
}
