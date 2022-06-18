func climbStairs(n int) int {
	a := make([]int, n)
	for i := 0; i < n; i++ {
		switch i {
		case 0:
			a[i] = 1
		case 1:
			a[i] = 2
		default:
			a[i] = a[i-2] + a[i-1]
		}
	}
	return a[n-1]
}
