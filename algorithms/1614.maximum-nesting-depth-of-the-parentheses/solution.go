func maxDepth(s string) int {
	max := 0
	current := 0

	for _, c := range s {
		if c == '(' {
			current += 1
			if current > max {
				max = current
			}
		} else if c == ')' {
			if current > 0 {
				current -= 1
			}
		}
	}

	return max
}
