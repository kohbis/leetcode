impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s: String = s
            .to_lowercase()
            .chars()
            .filter(char::is_ascii_alphanumeric)
            .collect();

        s.chars()
            .zip(s.chars().rev())
            .fold(true, |valid, (a, b)| valid && (a == b))
    }
}
