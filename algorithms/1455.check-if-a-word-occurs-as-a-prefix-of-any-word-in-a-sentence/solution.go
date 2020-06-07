func isPrefixOfWord(sentence string, searchWord string) int {
	for index, word := range strings.Split(sentence, " ") {
		if strings.HasPrefix(word, searchWord) {
			return index + 1;
		}
	}
	return -1;
}
