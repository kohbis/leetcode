func rotateString(A string, B string) bool {
	if A == "" && B == "" {
		return true
	}

	r := strings.Split(A, "")
	for i := 0; i < len(A); i++ {
		if B == strings.Join(append(r[i:], r[:i]...), "") {
			return true
		}
	}

	return false
}
