func checkTwoChessboards(coordinate1 string, coordinate2 string) bool {
	x1 := int(coordinate1[0] - 'a' + 1)
	y1 := int(coordinate1[1] - '0')
	x2 := int(coordinate2[0] - 'a' + 1)
	y2 := int(coordinate2[1] - '0')
	return (x1+y1)%2 == (x2+y2)%2
}
