func longestPalindrome(s string) string {
	runes := []rune(s)
	left, right := 0, 0
	longest := 0

	for i := 0; i < len(s)-1; i++ {
		for j := i; j < len(s); j++ {
			if longest > j-i {
				continue
			}

			if isPalindrome(runes, i, j) {
				longest = j - i
				left, right = i, j
			}
		}
	}

	return s[left : right+1]
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
