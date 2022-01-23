func convert(s string, numRows int) string {
	sLen := len(s)

	if numRows == 1 || sLen <= numRows {
		return s
	}

	rows := make([][]rune, numRows)
	row := 0
	i := 1

	for _, r := range s {
		rows[row] = append(rows[row], r)

		if row == 0 {
			i = 1
		} else if row == numRows-1 {
			i = -1
		}

		row = row + i
	}

	res := ""
	for _, runes := range rows {
		res += string(runes)
	}

	return res
}
