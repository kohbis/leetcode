func mostWordsFound(sentences []string) int {
	max := 0

	for _, s := range sentences {
		count := wordCount(s)
		if count > max {
			max = count
		}
	}

	return max
}

func wordCount(s string) int {
	words := strings.Fields(s)
	return len(words)
}
