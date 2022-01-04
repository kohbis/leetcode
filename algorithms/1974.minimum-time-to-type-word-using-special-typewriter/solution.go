func minTimeToType(word string) int {
	prev := 0
	time := 0

	for _, r := range word {
		curr := int(r - 'a')
		diff := int(math.Abs(float64(curr - prev)))

		if diff > 26-diff {
			time += 26 - diff
		} else {
			time += diff
		}
		time += 1

		prev = curr
	}

	return time
}

