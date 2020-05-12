func findJudge(N int, trust [][]int) int {
	if len(trust) == 0 {
		return 1
	}
	people := make([]int, N+1)
	for _, t := range trust {
		people[t[0]]--
		people[t[1]]++
	}
	for idx, p := range people {
		if p == N-1 {
			return idx
		}
	}
	return -1
}
