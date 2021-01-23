func maxIncreaseKeepingSkyline(grid [][]int) int {
	length := len(grid)
	row := make(map[int]int)
	col := make(map[int]int)

	for i := 0; i < length; i++ {
		for j := 0; j < length; j++ {
			val := grid[i][j]

			if val > row[i] {
				row[i] = val
			}
			if val > col[j] {
				col[j] = val
			}
		}
	}

	res := 0
	for i := 0; i < length; i++ {
		for j := 0; j < length; j++ {
			if col[j] > row[i] {
				res += row[i] - grid[i][j]
			} else {
				res += col[j] - grid[i][j]
			}
		}
	}

	return res
}
