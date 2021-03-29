func secondHighest(s string) int {
	digits := "0123456789"

	m := make(map[rune]bool)
	nums := []int{}
	for _, r := range s {
		if strings.ContainsRune(digits, r) {
			if !m[r] {
				m[r] = true
				nums = append(nums, int(r)-'0')
			}
		}
	}

	sort.Slice(nums, func(i, j int) bool {
		return nums[i] > nums[j]
	})

	if len(nums) > 1 {
		return nums[1]
	} else {
		return -1
	}
}
