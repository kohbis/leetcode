func isSameAfterReversals(num int) bool {
	if num == 0 || num%10 != 0 {
		return true
	}

	return false
}
