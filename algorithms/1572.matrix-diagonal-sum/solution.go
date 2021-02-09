func diagonalSum(mat [][]int) int {
	sum := 0
	l := len(mat)
	i := l / 2

	if l%2 == 1 {
		sum += mat[i][i]
	}

	i -= 1

	for i >= 0 {
		sum += mat[i][i] + mat[i][l-i-1] + mat[l-i-1][i] + mat[l-i-1][l-i-1]
		i -= 1
	}

	return sum
}
