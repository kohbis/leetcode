func thirdMax(nums []int) int {
	tops := []int{}

	for _, n := range nums {
		switch len(tops) {
		case 0:
			tops = append(tops, n)
		case 1:
			if n > tops[0] {
				tops = append(tops, tops[0])
				tops[0] = n
			} else if tops[0] > n {
				tops = append(tops, n)
			}
		case 2:
			if n > tops[0] {
				tops = append(tops, tops[1])
				tops[0], tops[1] = n, tops[0]
			} else if tops[0] > n && n > tops[1] {
				tops = append(tops, tops[1])
				tops[1] = n
			} else if tops[1] > n {
				tops = append(tops, n)
			}
		case 3:
			if n > tops[0] {
				tops[0], tops[1], tops[2] = n, tops[0], tops[1]
			} else if tops[0] > n && n > tops[1] {
				tops[1], tops[2] = n, tops[1]
			} else if tops[1] > n && n > tops[2] {
				tops[2] = n
			}
		}
	}

	if len(tops) < 3 {
		return tops[0]
	}

	return tops[2]
}
