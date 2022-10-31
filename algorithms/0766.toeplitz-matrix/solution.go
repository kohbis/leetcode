func isToeplitzMatrix(matrix [][]int) bool {
	rows := len(matrix)
	cols := len(matrix[0])

	for m := 1; m < rows; m++ {
		for n := 1; n < cols; n++ {
			if matrix[m][n] != matrix[m-1][n-1] {
				return false
			}
		}
	}

	return true
}
