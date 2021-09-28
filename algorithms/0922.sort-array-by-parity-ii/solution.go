func sortArrayByParityII(nums []int) []int {
	odd := []int{}
	even := []int{}
	for _, n := range nums {
		if n%2 == 0 {
			even = append(even, n)
		} else {
			odd = append(odd, n)
		}
	}

	res := []int{}
	for len(odd) > 0 || len(even) > 0 {
		if len(even) > 0 {
			res = append(res, even[len(even)-1])
			even = even[:len(even)-1]
		}
		if len(odd) > 0 {
			res = append(res, odd[len(odd)-1])
			odd = odd[:len(odd)-1]
		}
	}

	return res
}
