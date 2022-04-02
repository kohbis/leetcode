func validPalindrome(s string) bool {
	runes := []rune(s)
	left, right := 0, len(s)-1

	for left < right {
		if runes[left] != runes[right] {
			return isPalindrome(runes, left+1, right) || isPalindrome(runes, left, right-1)
		}

		left++
		right--
	}

	return true
}

func isPalindrome(runes []rune, left int, right int) bool {
	for left < right {
		if runes[left] != runes[right] {
			return false
		}

		left++
		right--
	}

	return true
}
