func minimumSum(num int) int {
	nums := []rune(strconv.Itoa(num))
	sort.Slice(nums, func(i, j int) bool {
		return nums[i] < nums[j]
	})

	ans := 0
	for i, r := range nums {
		n := int(r) - '0'
		if i > 1 {
			ans += n
		} else {
			ans += n * 10
		}
	}

	return ans
}
