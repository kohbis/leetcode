func firstPalindrome(words []string) string {
	for _, word := range words {
		if isPalindrome(word) {
			return word
		}
	}

	return ""
}

func isPalindrome(word string) bool {
	r := []rune(word)
	left, right := 0, len(word)-1

	for left < right {
		if r[left] != r[right] {
			return false
		}

		left++
		right--
	}

	return true
}
