func sumOfUnique(nums []int) int {
	sum := 0
	m := map[int]int{}

	for _, n := range nums {
		if _, ok := m[n]; ok {
			m[n] += 1
		} else {
			m[n] = 1
		}
	}

	for k, v := range m {
		if v == 1 {
			sum += k
		}
	}

	return sum
}
