func isAnagram(s string, t string) bool {
	ss := strings.Split(s, "")
	ts := strings.Split(t, "")

	sort.Strings(ss)
	sort.Strings(ts)

	return strings.Join(ss, "") == strings.Join(ts, "")
}
