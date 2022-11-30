func areOccurrencesEqual(s string) bool {
	count := map[rune]int{}

	for _, r := range s {
		if _, ok := count[r]; ok {
			count[r] += 1
		} else {
			count[r] = 1
		}
	}

	n := 0
	for _, v := range count {
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
