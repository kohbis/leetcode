func nextGreatestLetter(letters []byte, target byte) byte {
	res := letters[0]
	for i := 0; i < len(letters); i++ {
		if target < letters[i] {
			res = letters[i]
			break
		}
	}
	return res
}
