func reverseVowels(s string) string {
	vowels := []rune{'a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'}
	stack := []rune{}
	for _, r := range s {
		if contains(vowels, r) {
			stack = append(stack, r)
		}
	}

	res := ""
	for _, r := range s {
		if contains(vowels, r) {
			x := stack[len(stack)-1]
			stack = stack[:len(stack)-1]

			res += string(x)
		} else {
			res += string(r)
		}
	}

	return res
}

func contains(arr []rune, x rune) bool {
	for _, v := range arr {
		if v == x {
			return true
		}
	}
	return false
}
