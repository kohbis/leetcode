func findLonely(nums []int) []int {
	m := map[int]int{}
	for _, num := range nums {
		if _, ok := m[num]; ok {
			m[num] += 1
		} else {
			m[num] = 1
		}
	}

	ans := []int{}
	for key, value := range m {
		if value == 1 {
			_, prev := m[key-1]
			_, next := m[key+1]
			if !prev && !next {
				ans = append(ans, key)
			}
		}
	}

	return ans
}
