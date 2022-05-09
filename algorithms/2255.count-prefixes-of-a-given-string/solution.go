func countPrefixes(words []string, s string) int {
	count := 0
	for _, word := range words {
		l := len(word)

		if l > len(s) {
			continue
		}

		if word == s[:l] {
			count++
		}
	}
	return count
}
