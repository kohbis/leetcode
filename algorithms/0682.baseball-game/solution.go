func calPoints(ops []string) int {
	scores := []int{}

	for _, op := range ops {
		switch op {
		case "C":
			scores = scores[:len(scores)-1]
		case "D":
			score := scores[len(scores)-1] * 2
			scores = append(scores, score)
		case "+":
			score := scores[len(scores)-2] + scores[len(scores)-1]
			scores = append(scores, score)
		default:
			score, _ := strconv.Atoi(op)
			scores = append(scores, score)
		}
	}

	sum := 0
	for _, score := range scores {
		sum += score
	}
	return sum
}
