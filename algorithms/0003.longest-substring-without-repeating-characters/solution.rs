use std::cmp::max;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut curr: Vec<char> = Vec::new();
        let mut longest = 0;

        for c in s.chars() {
            if let Some(i) = curr.iter().position(|&x| x == c) {
                curr.drain(0..(i + 1));
            }

            curr.push(c);
            longest = max(longest, curr.len());
        }

        longest as i32
    }
}
