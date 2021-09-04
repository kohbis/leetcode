func processQueries(queries []int, m int) []int {
	p := make([]int, m)
	for i := range p {
		p[i] = i + 1
	}

	res := []int{}
	for _, n := range queries {
		idx := indexOf(n, p)
		res = append(res, idx)

		p = append(p[:idx], p[idx+1:]...)
		p = append([]int{n}, p...)
	}

	return res
}

func indexOf(n int, p []int) int {
	for i, x := range p {
		if n == x {
			return i
		}
	}
	return -1
}
