func isPrefixString(s string, words []string) bool {
	var curr string

	for _, word := range words {
		curr += word

		if s == curr {
			return true
		}

		if len(s) <= len(curr) {
			return false
		}
	}

	return false
}
