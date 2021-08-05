func distributeCandies(candies int, num_people int) []int {
	dist := make([]int, num_people)

	takes := 0
	for candies > 0 {
		idx := takes % num_people

		if candies > takes+1 {
			dist[idx] += takes + 1
		} else {
			dist[idx] += candies
		}

		takes += 1
		candies -= takes
	}

	return dist
}
