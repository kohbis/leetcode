func finalValueAfterOperations(operations []string) int {
	res := 0

	for _, op := range operations {
		switch op {
		case "++X", "X++":
			res++
		case "--X", "X--":
			res--
		}
	}

	return res
}

