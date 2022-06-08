func removePalindromeSub(s string) int {
	if len(s) == 0 {
		return 0
	} else if s == reverse(s) {
		return 1
	} else {
		return 2
	}
}

func reverse(s string) (res string) {
	for _, v := range s {
		res = string(v) + res
	}
	return
}
