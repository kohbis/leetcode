impl Solution {
    pub fn remove_trailing_zeros(num: String) -> String {
        let mut digits: Vec<char> = num.chars().collect();
        for i in (0..num.len()).rev() {
            if digits[i] == '0' {
                digits.pop();
            } else {
                break;
            }
        }
        digits.iter().collect()
    }
}
