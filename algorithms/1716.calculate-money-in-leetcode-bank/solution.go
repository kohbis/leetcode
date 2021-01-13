func totalMoney(n int) int {
	total := 0
	week := n / 7
	extraDay := n % 7

	total += 28 * week
	total += week * (week - 1) / 2 * 7
	total += week * extraDay
	total += extraDay * (extraDay + 1) / 2

	return total
}
