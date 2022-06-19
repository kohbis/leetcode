
func matrixReshape(mat [][]int, r int, c int) [][]int {
	if len(mat[0])*len(mat) != r*c {
		return mat
	}

	data := []int{}
	for i := 0; i < len(mat); i++ {
		data = append(data, mat[i]...)
	}

	reshaped := make([][]int, r)
	for i := 0; i < r; i++ {
		reshaped[i] = make([]int, c)
		for j := 0; j < c; j++ {
			reshaped[i][j] = data[i*c+j]
		}
	}

	return reshaped
}
