func average(salary []int) float64 {
	min := salary[0]
	max := 0
	l := len(salary)
	sum := 0

	for _, s := range salary {
		if s < min {
			min = s
		}
		if s > max {
			max = s
		}
		sum += s
	}

	return float64(sum-min-max) / float64(l-2)
}
