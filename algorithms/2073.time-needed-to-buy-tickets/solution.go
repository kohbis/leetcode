func timeRequiredToBuy(tickets []int, k int) int {
	seconds := 0

	for {
		for i := 0; i < len(tickets); i++ {
			if tickets[i] > 0 {
				seconds++
				tickets[i]--

				if k == i && tickets[i] == 0 {
					return seconds
				}
			}
		}
	}

	return seconds
}
