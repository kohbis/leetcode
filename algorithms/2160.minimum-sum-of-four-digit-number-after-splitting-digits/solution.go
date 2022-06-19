func minimumSum(num int) int {
	nums := []rune(strconv.Itoa(num))
	sort.Slice(nums, func(i, j int) bool {
		return nums[i] < nums[j]
	})

	res := 0
	for i, r := range nums {
		n := int(r) - '0'
		if i > 1 {
			res += n
		} else {
			res += n * 10
		}
	}

	return res
}
