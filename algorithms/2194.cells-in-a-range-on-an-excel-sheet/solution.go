func cellsInRange(s string) []string {
	rowStart, rowEnd := s[0]-64, s[3]-64
	colStart, _ := strconv.Atoi(string(s[1]))
	colEnd, _ := strconv.Atoi(string(s[4]))

	list := []string{}
	for i := rowStart; i <= rowEnd; i++ {
		row := string('A' - 1 + i)
		for j := colStart; j <= colEnd; j++ {
			list = append(list, row+strconv.Itoa(j))
		}
	}

	return list
}
