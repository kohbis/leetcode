func defangIPaddr(address string) string {
	res := ""
	for _, r := range address {
		if string(r) == "." {
			res += "[.]"
		} else {
			res += string(r)
		}
	}
	return res
}
