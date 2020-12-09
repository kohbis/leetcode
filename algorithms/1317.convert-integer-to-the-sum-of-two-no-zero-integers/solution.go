func getNoZeroIntegers(n int) []int {
	var res []int

	for i := 1; i < n; i++ {
		if noZero(i) && noZero(n-i) {
			res = []int{i, n - i}
			break
		}
	}

	return res
}

func noZero(num int) bool {
	for num > 0 {
		if num%10 == 0 {
			return false
		}

		num /= 10
	}

	return true
}
