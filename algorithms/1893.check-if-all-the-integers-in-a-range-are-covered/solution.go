func isCovered(ranges [][]int, left int, right int) bool {
	covered := make([]bool, 51)

	for _, v := range ranges {
		for i := v[0]; i <= v[1]; i++ {
			covered[i] = true
		}
	}

	for i := left; i <= right; i++ {
		if !covered[i] {
			return false
		}
	}

	return true
}
