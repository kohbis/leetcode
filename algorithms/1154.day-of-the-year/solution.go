func dayOfYear(date string) int {
	year_month_day := make([]int, 3)
	for i, s := range strings.Split(date, "-") {
		year_month_day[i], _ = strconv.Atoi(s)
	}

	days := []int{
		0,
		31, // Jan.
		28, // Feb.
		31, // Mar.
		30, // Apr.
		31, // May.
		30, // June
		31, // July
		31, // Aug.
		30, // Sept.
		31, // Oct.
		30, // Nov.
		31, // Dec.
	}

	if leapYear(year_month_day[0]) {
		days[2] = 29
	}

	sum := 0
	for i := 0; i < year_month_day[1]; i++ {
		sum += days[i]
	}

	return sum + year_month_day[2]
}

func leapYear(year int) bool {
	return (year%4 == 0) && !(year%100 == 0) || (year%400 == 0)
}
