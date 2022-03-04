func prefixCount(words []string, pref string) int {
	count := 0

	plen := len(pref)
	for _, word := range words {
		if len(word) >= plen && word[:plen] == pref {
			count++
		}
	}

	return count
}
