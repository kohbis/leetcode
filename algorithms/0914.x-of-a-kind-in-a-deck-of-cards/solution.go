func hasGroupsSizeX(deck []int) bool {
	m := map[int]int{}
	for _, d := range deck {
		if _, ok := m[d]; ok {
			m[d] += 1
		} else {
			m[d] = 1
		}
	}

	vs := []int{}
	for _, v := range m {
		vs = append(vs, v)
	}

	x := vs[0]
	for _, v := range vs {
		x = gcd(x, v)
	}

	return x >= 2
}

func gcd(x int, y int) int {
	if y == 0 {
		return x
	} else {
		return gcd(y, x%y)
	}
}
