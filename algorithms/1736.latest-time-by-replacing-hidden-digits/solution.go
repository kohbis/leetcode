func maximumTime(time string) string {
	digits := []rune(time)
	res := []rune{}

	// "*x:xx"
	if digits[0] != '?' {
		res = append(res, digits[0])
	} else {
		if digits[1] == '?' || int(digits[1]-'0') < 4 {
			res = append(res, '2')
		} else {
			res = append(res, '1')
		}
	}

	// "x*:xx"
	if digits[1] != '?' {
		res = append(res, digits[1])
	} else {
		if res[0] == '2' {
			res = append(res, '3')
		} else {
			res = append(res, '9')
		}
	}

	// "xx*xx"
	res = append(res, ':')

	// "xx:*x"
	if digits[3] != '?' {
		res = append(res, digits[3])
	} else {
		res = append(res, '5')
	}

	// "xx:x*"
	if digits[4] != '?' {
		res = append(res, digits[4])
	} else {
		res = append(res, '9')
	}

	return string(res)
}
