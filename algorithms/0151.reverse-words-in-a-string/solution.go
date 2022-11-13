func reverseWords(s string) string {
	res := ""
	for _, word := range strings.Fields(s) {
		if res == "" {
			res = word
		} else {
			res = word + " " + res
		}
	}
	return res
}
