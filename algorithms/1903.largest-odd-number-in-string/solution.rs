impl Solution {
    pub fn largest_odd_number(num: String) -> String {
        for (i, c) in num.chars().rev().enumerate() {
            let n = c as u8 - '0' as u8;
            if n % 2 == 1 {
                return num[..num.len() - i].to_string();
            }
        }

        "".to_string()
    }
}
