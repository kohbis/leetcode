func brokenCalc(startValue int, target int) int {
	count := 0

	for startValue < target {
		count++

		if target%2 == 0 {
			target /= 2
		} else {
			target++
		}
	}

	return count + startValue - target
}
