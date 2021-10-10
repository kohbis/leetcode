func maxRepeating(sequence string, word string) int {
	k := len(sequence) / len(word)

	for !strings.Contains(sequence, strings.Repeat(word, k)) {
		k--
	}

	return k
}
