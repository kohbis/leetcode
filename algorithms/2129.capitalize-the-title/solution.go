func capitalizeTitle(title string) string {
	words := []string{}

	for _, word := range strings.Split(title, " ") {
		if len(word) > 2 {
			tmp := strings.ToLower(word)
			words = append(words, string(tmp[0]-32)+tmp[1:])
		} else {
			words = append(words, strings.ToLower(word))
		}
	}
	return strings.Join(words, " ")
}
