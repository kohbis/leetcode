func mergeAlternately(word1 string, word2 string) string {
	runes1 := []rune(word1)
	runes2 := []rune(word2)

	res := []string{}
	for len(runes1) > 0 || len(runes2) > 0 {
		if len(runes1) > 0 {
			res = append(res, string(runes1[0]))
			runes1 = runes1[1:]
		}

		if len(runes2) > 0 {
			res = append(res, string(runes2[0]))
			runes2 = runes2[1:]
		}
	}

	return strings.Join(res, "")
}

