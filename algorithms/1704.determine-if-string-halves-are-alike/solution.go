func halvesAreAlike(s string) bool {
	vowels := "aeiouAEIOU"

	mid := len(s) / 2
	aVowels := 0
	bVowels := 0

	for i, r := range s {
		if strings.ContainsRune(vowels, r) {
			if i < mid {
				aVowels += 1
			} else {
				bVowels += 1
			}
		}
	}

	return aVowels == bVowels
}
