func findGCD(nums []int) int {
	min := 1001
	max := 0

	for _, n := range nums {
		if n < min {
			min = n
		}

		if n > max {
			max = n
		}
	}

	return gcd(max, min)
}

func gcd(x int, y int) int {
	if y == 0 {
		return x
	}

	return gcd(y, x%y)
}
