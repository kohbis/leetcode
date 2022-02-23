func majorityElement(nums []int) int {
	m := map[int]int{}
	for _, n := range nums {
		if _, ok := m[n]; ok {
			m[n] += 1
		} else {
			m[n] = 1
		}
	}

	ans := 0
	for k, v := range m {
		if v > m[ans] {
			ans = k
		}
	}

	return ans

}
