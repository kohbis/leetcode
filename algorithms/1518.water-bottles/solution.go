func numWaterBottles(numBottles int, numExchange int) int {
	count := 0
	full := numBottles
	empty := 0

	for 0 < full {
		count += full
		empty += full
		full = empty / numExchange
		empty = empty % numExchange
	}

	return count
}
