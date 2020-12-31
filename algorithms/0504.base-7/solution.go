func convertToBase7(num int) string {
	if num == 0 {
		return "0"
	}

	var res string
	var n int
	if num > 0 {
		n = num
	} else {
		n = num * -1
	}

	for n > 0 {
		rem := n % 7
		n /= 7
		res = strconv.Itoa(rem) + res
	}

	if num > 0 {
		return res
	} else {
		return "-" + res
	}
}
