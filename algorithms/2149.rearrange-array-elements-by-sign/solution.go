func rearrangeArray(nums []int) []int {
	pos, neg := []int{}, []int{}

	for _, n := range nums {
		if n > 0 {
			pos = append(pos, n)
		} else {
			neg = append(neg, n)
		}
	}

	ans := []int{}
	for i := 0; i < len(nums)/2; i++ {
		ans = append(ans, pos[i], neg[i])
	}
	return ans
}
