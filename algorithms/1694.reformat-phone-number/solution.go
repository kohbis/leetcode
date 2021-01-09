func reformatNumber(number string) string {
	parts := []string{}
	nums := []rune{}

	for _, r := range number {
		if r != '-' && r != ' ' {
			nums = append(nums, r)
		}
	}

	threeDigits := len(nums) / 3
	twoDigits := 0
	switch len(nums) % 3 {
	case 1:
		threeDigits -= 1
		twoDigits = 2
	case 2:
		twoDigits = 1
	default:
		twoDigits = 0
	}

	for i := 0; i < threeDigits; i++ {
		var s string

		s += string(nums[0:3])
		nums = nums[3:]

		parts = append(parts, s)
	}

	for i := 0; i < twoDigits; i++ {
		var s string

		s += string(nums[0:2])
		nums = nums[2:]

		parts = append(parts, s)
	}

	return strings.Join(parts, "-")
}

