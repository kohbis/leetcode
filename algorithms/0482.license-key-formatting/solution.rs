impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        s.to_uppercase()
            .chars()
            .filter(|&c| c != '-')
            .rev()
            .collect::<Vec<char>>()
            .chunks(k as usize)
            .map(|parts| parts.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("-")
            .chars()
            .rev()
            .collect::<String>()
    }
}
