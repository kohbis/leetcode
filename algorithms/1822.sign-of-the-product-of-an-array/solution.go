func arraySign(nums []int) int {
	negativeCount := 0
	for _, n := range nums {
		if n == 0 {
			return 0
		}
		if 0 > n {
			negativeCount += 1
		}
	}

	if negativeCount%2 == 0 {
		return 1
	} else {
		return -1
	}
}
