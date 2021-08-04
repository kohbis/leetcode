use std::cmp::max;

impl Solution {
    pub fn check_zero_ones(s: String) -> bool {
        let mut longest_zero = 0;
        let mut longest_one = 0;
        let mut curr_zero = 0;
        let mut curr_one = 0;

        for c in s.chars() {
            if c == '0' {
                curr_one = 0;
                curr_zero += 1;
                longest_zero = max(longest_zero, curr_zero);
            } else {
                curr_zero = 0;
                curr_one += 1;
                longest_one = max(longest_one, curr_one);
            }
        }

        longest_zero < longest_one
    }
}
