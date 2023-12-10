func transpose(matrix [][]int) [][]int {
	rows := len(matrix)
	cols := len(matrix[0])

	res := make([][]int, cols)
	for row := 0; row < cols; row++ {
		res[row] = make([]int, rows)
		for col := 0; col < rows; col++ {
			res[row][col] = matrix[col][row]
		}
	}

	return res
}
