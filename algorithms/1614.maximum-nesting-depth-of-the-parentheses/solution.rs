impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut max: i32 = 0;
        let mut current: i32 = 0;

        for c in s.chars() {
            if c == '(' {
                current += 1;
                if current > max {
                    max = current;
                }
            } else if c == ')' {
                if current > 0 {
                    current -= 1;
                }
            }
        }

        max
    }
}
