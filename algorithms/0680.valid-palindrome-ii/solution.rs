impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let chars = s.chars().collect::<Vec<char>>();
        let (mut left, mut right) = (0, chars.len() - 1);

        while left < right {
            if chars[left] != chars[right] {
                return Self::is_palindrome(&chars, left + 1, right)
                    || Self::is_palindrome(&chars, left, right - 1);
            }

            left += 1;
            right -= 1;
        }

        true
    }

    fn is_palindrome(chars: &Vec<char>, mut left: usize, mut right: usize) -> bool {
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
