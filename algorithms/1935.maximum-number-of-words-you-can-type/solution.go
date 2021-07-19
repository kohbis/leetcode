func canBeTypedWords(text string, brokenLetters string) int {
	var count int
	brokenRunes := []rune(brokenLetters)

	outer:
	for _, word := range strings.Fields(text) {
		for _, r := range brokenRunes {
			if strings.ContainsRune(word, r) {
				continue outer
			}
		}
		count++
	}

	return count
}

