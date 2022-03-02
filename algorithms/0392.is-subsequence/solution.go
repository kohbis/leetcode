func isSubsequence(s string, t string) bool {
	letters := []rune(t)

	for _, r := range s {
		i := indexOf(r, letters)
		if i >= 0 {
			letters = letters[i+1:]
		} else {
			return false
		}
	}

	return true
}

func indexOf(target rune, runes []rune) int {
	for i, r := range runes {
		if r == target {
			return i
		}
	}
	return -1
}
