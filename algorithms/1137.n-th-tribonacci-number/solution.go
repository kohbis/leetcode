func tribonacci(n int) int {
	if n <= 1 {
		return n
	}

	tri := [3]int{0, 0, 1}
	for i := 2; i < n+1; i++ {
		sum := tri[0] + tri[1] + tri[2]

		tri[0] = tri[1]
		tri[1] = tri[2]
		tri[2] = sum
	}

	return tri[2]
}
