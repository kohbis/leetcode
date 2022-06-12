impl Solution {
    pub fn strong_password_checker_ii(password: String) -> bool {
        let chars: Vec<char> = password.chars().collect();

        if chars.len() < 8 {
            return false;
        }

        let specials: Vec<char> = vec!['!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '+'];

        let mut lower = false;
        let mut upper = false;
        let mut digit = false;
        let mut special = false;
        for i in 0..chars.len() {
            if chars[i].is_lowercase() {
                lower = true;
            } else if chars[i].is_uppercase() {
                upper = true;
            } else if chars[i].is_digit(10) {
                digit = true;
            } else if specials.contains(&chars[i]) {
                special = true;
            } else {
                return false;
            }

            if i > 0 && chars[i - 1] == chars[i] {
                return false;
            }
        }

        lower && upper && digit && special
    }
}
