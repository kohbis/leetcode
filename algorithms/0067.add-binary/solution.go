func addBinary(a string, b string) string {
	i, j, carry := len(a), len(b), 0
	var res string

	for i > 0 || j > 0 || carry > 0 {
		if i > 0 {
			i--
			carry += int(a[i] - '0')
		}
		if j > 0 {
			j--
			carry += int(b[j] - '0')
		}

		res = strconv.Itoa(carry%2) + res
		carry /= 2
	}

	return res
}
