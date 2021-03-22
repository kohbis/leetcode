func maxAscendingSum(nums []int) int {
	sums := []int{}
	tmp := []int{}

	for _, n := range nums {
		if len(tmp) > 0 {
			if tmp[len(tmp)-1] >= n {
				sums = append(sums, sum(tmp))
				tmp = []int{}
			}
		}
		tmp = append(tmp, n)
	}
	sums = append(sums, sum(tmp))

	max := 0
	for _, sum := range sums {
		if sum > max {
			max = sum
		}
	}
	return max
}

func sum(nums []int) int {
	sum := 0
	for _, n := range nums {
		sum += n
	}
	return sum
}
