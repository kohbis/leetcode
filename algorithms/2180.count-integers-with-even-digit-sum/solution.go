func countEven(num int) int {
	ans := 0

	for i := 1; i <= num; i++ {
		if digitSum(i)%2 == 0 {
			ans++
		}
	}

	return ans
}

func digitSum(num int) int {
	sum := 0
	for _, r := range strconv.Itoa(num) {
		sum += int(r - '0')
	}

	return sum
}
