impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        let (mut left, mut right) = (0, 0);
        let mut longest = 0;
        for i in 0..(s.len() - 1) {
            for j in (i + 1)..s.len() {
                if longest > j - i {
                    continue;
                }

                if Self::is_palindrome(&chars, i, j) {
                    longest = j - i;
                    left = i;
                    right = j;
                }
            }
        }

        s[left..=right].to_string()
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
