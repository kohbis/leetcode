func countConsistentStrings(allowed string, words []string) int {
    count := 0

    for _, word := range words {
        onlyAllowedChars := true

        for _, c := range word {
            if !strings.ContainsRune(allowed, c) {
                onlyAllowedChars = false
                break
            }
        }

        if onlyAllowedChars {
            count += 1
        }
    }

    return count
}
