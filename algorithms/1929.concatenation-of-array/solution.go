func getConcatenation(nums []int) []int {
	l := len(nums)
	res := make([]int, l*2)
	for i, n := range nums {
		res[i] = n
		res[i+l] = n
	}
	return res
}
