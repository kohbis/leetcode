func countBalls(lowLimit int, highLimit int) int {
	m := map[int]int{}

	for i := lowLimit; i < highLimit+1; i++ {
		sum := digitsSum(i)
		if _, ok := m[sum]; ok {
			m[sum] += 1
		} else {
			m[sum] = 1
		}
	}

	max := 0
	for _, v := range m {
		if v > max {
			max = v
		}
	}

	return max
}

func digitsSum(num int) int {
	sum := 0

	for _, s := range strings.Split(strconv.Itoa(num), "") {
		i, _ := strconv.Atoi(s)
		sum += i
	}

	return sum
}
