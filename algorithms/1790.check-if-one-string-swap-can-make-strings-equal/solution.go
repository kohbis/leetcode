func areAlmostEqual(s1 string, s2 string) bool {
	a, b := []rune{}, []rune{}
	runes1, runes2 := []rune(s1), []rune(s2)

	for i := 0; i < len(s1); i++ {
		if s1[i] != s2[i] {
			a = append(a, runes1[i])
			b = append(b, runes2[i])

			if len(a) > 2 {
				return false
			}
		}
	}

	sort.Slice(a, func(i, j int) bool {
		return a[i] < a[j]
	})
	sort.Slice(b, func(i, j int) bool {
		return b[i] < b[j]
	})
	return string(a) == string(b)
}
