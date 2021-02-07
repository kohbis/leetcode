func minOperations(logs []string) int {
	depth := 0
	for _, log := range logs {
		if "../" == log {
			if depth > 0 {
				depth -= 1
			}
		} else if "./" != log {
			depth += 1
		}
	}
	return depth
}
