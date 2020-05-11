func twoSum(nums []int, target int) []int {
	m := map[int]int{}
	for idx, n := range nums {
		diff := target - n
		if i, ok := m[diff]; ok {
			return []int{i, idx}
		} else {
			m[n] = idx
		}
	}
	return []int{}
}
