func checkString(s string) bool {
	appeared := false
	for _, r := range s {
		switch r {
		case 'a':
			if appeared {
				return false
			}
		case 'b':
			appeared = true
		}
	}

	return true
}
