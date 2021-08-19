func checkRecord(s string) bool {
	a := 0
	l := 0

	for _, r := range s {
		switch r {
		case 'A':
			a++
			l = 0
		case 'L':
			l++
		default:
			l = 0
		}

		if a >= 2 || l >= 3 {
			return false
		}
	}

	return true
}
