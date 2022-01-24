func detectCapitalUse(word string) bool {
	runes := []rune(word)
	count := 0
	for _, r := range runes {
		if unicode.IsUpper(r) {
			count++
		}
	}

	return count == 0 || count == len(word) || (count == 1 && unicode.IsUpper(runes[0]))
}

