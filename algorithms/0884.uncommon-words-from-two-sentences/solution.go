func uncommonFromSentences(A string, B string) []string {
	m := map[string]int{}

	for _, sentence := range [...]string{A, B} {
		for _, word := range strings.Fields(sentence) {
			if _, ok := m[word]; ok {
				m[word] += 1
			} else {
				m[word] = 1
			}
		}
	}

	res := []string{}
	for k, v := range m {
		if v == 1 {
			res = append(res, k)
		}
	}
	return res
}

