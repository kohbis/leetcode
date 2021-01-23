func minPartitions(n string) int {
	var m int

	for _, s := range strings.Split(n, "") {
		i, _ := strconv.Atoi(s)
		if i > m {
			m = i
		}
	}

	return m
}
