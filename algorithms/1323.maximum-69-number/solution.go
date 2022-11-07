func maximum69Number(num int) int {
	s := strconv.Itoa(num)
	s = strings.Replace(s, "6", "9", 1)
	i, _ := strconv.Atoi(s)
	return i
}
