func maximumPopulation(logs [][]int) int {
	m := map[int]int{}

	maxPopYear := logs[0][0]
	maxPop := 0
	for _, log := range logs {
		for year := log[0]; year < log[1]; year++ {
			if _, ok := m[year]; ok {
				m[year] += 1
			} else {
				m[year] = 1
			}

			if maxPop < m[year] {
				maxPop = m[year]
				maxPopYear = year
			} else if maxPop == m[year] {
				if year < maxPopYear {
					maxPopYear = year
				}
			}
		}
	}

	return maxPopYear
}
