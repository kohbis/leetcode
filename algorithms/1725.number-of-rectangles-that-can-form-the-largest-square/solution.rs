impl Solution {
    pub fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
        use std::cmp::min;

        let mut maxLen = 0;
        let mut maxLenCount = 0;

        for r in rectangles {
            let minLen = min(r[0], r[1]);

            if minLen > maxLen {
                maxLen = minLen;
                maxLenCount = 1
            } else if minLen == maxLen {
                maxLenCount += 1
            }
        }

        maxLenCount
    }
}
