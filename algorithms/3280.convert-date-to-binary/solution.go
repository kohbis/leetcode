func convertDateToBinary(date string) string {
	var res []string
	for _, part := range strings.Split(date, "-") {
		num, _ := strconv.Atoi(part)
		res = append(res, fmt.Sprintf("%b", num))
	}
	return strings.Join(res, "-")
}

