func getLucky(s string, k int) int {
	var sum int

	for _, r := range s {
		n := int(r - 'a' + 1)
		for n != 0 {
			sum += n % 10
			n /= 10
		}
	}

	for i := 1; i < k; i++ {
		curr := sum
		sum = 0

		for curr != 0 {
			sum += int(curr % 10)
			curr /= 10
		}
	}

	return sum
}
