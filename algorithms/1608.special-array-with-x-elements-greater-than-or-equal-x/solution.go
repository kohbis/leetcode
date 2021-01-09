func specialArray(nums []int) int {
	m := map[int]int{}
	for _, n := range nums {
		if _, ok := m[n]; ok {
			m[n] += 1
		} else {
			m[n] = 1
		}
	}

	for x := 1; x <= len(nums); x++ {
		sum := 0

		for k, v := range m {
			if k >= x {
				sum += v
			}
		}

		if x == sum {
			return x
		}
	}

	return -1
}


