func reverseString(s []byte) {
	if len(s) > 0 {
		i, j := 0, len(s)-1

		for i < j {
			buffer := s[i]
			s[i] = s[j]
			s[j] = buffer

			i++
			j--
		}
	}
}
