func kidsWithCandies(candies []int, extraCandies int) []bool {
	maxCandies := 0
	res := make([]bool, len(candies))

	for _, v := range candies {
		if v > maxCandies {
			maxCandies = v
		}
	}

	for i, v := range candies {
		if v+extraCandies >= maxCandies {
			res[i] = true
		}
	}

	return res
}
