func searchMatrix(matrix [][]int, target int) bool {
	colSize := len(matrix[0])
	for _, row := range matrix {
		if row[0] <= target && target <= row[colSize-1] {
			for _, x := range row {
				if x == target {
					return true
				}
			}

			break
		}
	}

	return false
}
