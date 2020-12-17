func interpret(command string) string {
	var s string
	s = strings.ReplaceAll(command, "()", "o")
	s = strings.ReplaceAll(s, "(al)", "al")
	return s
}
