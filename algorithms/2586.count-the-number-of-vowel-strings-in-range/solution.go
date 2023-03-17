func vowelStrings(words []string, left int, right int) int {
	res := 0
	for _, word := range words[left : right+1] {
		if isVowel(word[0]) && isVowel(word[len(word)-1]) {
			res++
		}
	}
	return res
}

func isVowel(b byte) bool {
	for _, v := range []byte("aeiou") {
		if v == b {
			return true
		}
	}
	return false
}
