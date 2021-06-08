impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        for i in 1..=(s.len() / 2) {
            if s.len() % i == 0 {
                if (&s[..i]).clone().repeat(s.len() / i) == s {
                    return true;
                }
            }
        }
        false
    }
}
