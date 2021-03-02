func countMatches(items [][]string, ruleKey string, ruleValue string) int {
	var index int
	for i, k := range []string{"type", "color", "name"} {
		if ruleKey == k {
			index = i
			break
		}
	}

	var count int
	for _, item := range items {
		if ruleValue == item[index] {
			count += 1
		}
	}
	return count
}
