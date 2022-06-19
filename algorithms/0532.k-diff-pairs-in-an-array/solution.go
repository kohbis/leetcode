func findPairs(nums []int, k int) int {
	m := map[int]int{}
	for _, n := range nums {
		if _, ok := m[n]; ok {
			m[n] += 1
		} else {
			m[n] = 1
		}
	}

	res := 0
	for key, value := range m {
		if _, ok := m[key+k]; ok {
			if (k == 0 && value > 1) || k > 0 {
				res++
			}
		}
	}

	return res
}
