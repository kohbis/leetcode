func titleToNumber(columnTitle string) int {
	var columnNumber int

	for _, r := range columnTitle {
		n := int(r - 'A' + 1)
		columnNumber = columnNumber*26 + n
	}

	return columnNumber
}
