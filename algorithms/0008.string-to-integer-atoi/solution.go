func myAtoi(s string) int {
	fields := strings.Fields(s)
	if len(fields) == 0 {
		return 0
	}

	sign, total, start := 1, 0, 0
	str := fields[0]
	first := string(str[0])
	if first == "+" || first == "-" {
		switch first {
		case "+":
			sign = 1
		case "-":
			sign = -1
		}
		start++
	}

	digits := []string{"0", "1", "2", "3", "4", "5", "6", "7", "8", "9"}
	for i := start; i < len(str); i++ {
		s, digit := string(str[i]), 0

		if contains(digits, s) {
			digit, _ = strconv.Atoi(s)
		} else {
			return sign * total
		}

		total = total*10 + digit

		if sign*total > math.MaxInt32 {
			return math.MaxInt32
		} else if sign*total < math.MinInt32 {
			return math.MinInt32
		}
	}

	return sign * total
}

func contains(arr []string, x string) bool {
	for _, v := range arr {
		if v == x {
			return true
		}
	}
	return false
}
