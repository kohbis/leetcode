func findTheDifference(s string, t string) byte {
	bytes := []byte(t)

	for _, b := range []byte(s) {
		i := indexOf(b, bytes)
		bytes = append(bytes[:i], bytes[i+1:]...)
	}

	return bytes[0]
}

func indexOf(target byte, bytes []byte) int {
	for i, b := range bytes {
		if b == target {
			return i
		}
	}
	return -1
}
