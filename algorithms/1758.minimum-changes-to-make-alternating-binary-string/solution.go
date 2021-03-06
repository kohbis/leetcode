func minOperations(s string) int {
        count1 := 0
        count2 := 0

        for i, c := range s {
                if i % 2 == 0 {
                        if c == '0' {
                                count1 += 1
                        } else {
                                // c == '1'
                                count2 += 1
                        }
                } else {
                        if c == '1' {
                                count1 += 1
                        } else {
                                // c == '0'
                                count2 += 1
                        }
                }
        }

        if count1 < count2 {
                return count1
        } else {
                return count2
        }
}
