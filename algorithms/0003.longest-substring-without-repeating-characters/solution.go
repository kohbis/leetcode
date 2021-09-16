func lengthOfLongestSubstring(s string) int {
	curr := []rune{}
	longest := 0

	for _, r := range s {
		for i := 0; i < len(curr); i++ {
			if curr[i] == r {
				curr = curr[i+1:]
			}
		}

		curr = append(curr, r)
		if len(curr) > longest {
			longest = len(curr)
		}
	}

	return longest
}
