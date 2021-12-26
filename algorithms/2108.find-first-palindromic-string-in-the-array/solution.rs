impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        for word in words {
            if Solution::is_palindrome(&word) {
                return word;
            }
        }

        "".to_string()
    }

    fn is_palindrome(s: &str) -> bool {
        let chars: Vec<char> = s.chars().collect();
        let (mut left, mut right) = (0, chars.len() - 1);

        while left < right {
            if chars[left] != chars[right] {
                return false;
            }

            left += 1;
            right -= 1;
        }

        true
    }
}
