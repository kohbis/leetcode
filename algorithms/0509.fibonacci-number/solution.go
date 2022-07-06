func fib(n int) int {
	a, b := 0, 1
	tmp := 0
	for i := 0; i < n; i++ {
		tmp = a + b
		a = b
		b = tmp
	}
	return a
}
