func makeFancyString(s string) string {
	res := []rune{}

	last := ' '
	second_last := ' '

	for _, r := range s {
		if r != last || r != second_last {
			res = append(res, r)
		}

		second_last = last
		last = r
	}

	return string(res)
}
