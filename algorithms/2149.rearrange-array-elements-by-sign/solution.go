func rearrangeArray(nums []int) []int {
	pos, neg := []int{}, []int{}

	for _, n := range nums {
		if n > 0 {
			pos = append(pos, n)
		} else {
			neg = append(neg, n)
		}
	}

	res := []int{}
	for i := 0; i < len(nums)/2; i++ {
		res = append(res, pos[i], neg[i])
	}
	return res
}
