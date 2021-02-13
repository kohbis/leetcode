func runningSum(nums []int) []int {
	var sum int
	var res []int

	for _, num := range nums {
		sum += num
		res = append(res, sum)
	}

	return res
}
