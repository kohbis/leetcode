func decode(encoded []int, first int) []int {
	res := []int{first}

	for i, v := range encoded {
		res = append(res, res[i]^v)
	}

	return res
}
