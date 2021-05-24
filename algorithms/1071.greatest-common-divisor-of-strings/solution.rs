impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        if str1 == str2 {
            return str1;
        } else if str2.len() > str1.len() {
            return Solution::gcd_of_strings(str2, str1);
        } else if str1.starts_with(&str2) {
            return Solution::gcd_of_strings(str1[str2.len()..].to_string(), str2);
        }

        "".to_string()
    }
}
