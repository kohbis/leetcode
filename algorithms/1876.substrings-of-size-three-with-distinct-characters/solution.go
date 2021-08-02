func countGoodSubstrings(s string) int {
	var count int

	for i := 2; i < len(s); i++ {
		if s[i-2] != s[i-1] && s[i-1] != s[i] && s[i] != s[i-2] {
			count++
		}
	}

	return count
}
