func reverse(x int) int {
	res := 0

	for x != 0 {
		res = res*10 + x%10
		x /= 10
	}

	if res < math.MinInt32 || math.MaxInt32 < res {
		return 0
	} else {
		return res
	}
}
