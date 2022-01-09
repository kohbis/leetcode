impl Solution {
    pub fn capitalize_title(title: String) -> String {
        title
            .split_whitespace()
            .map(|s| Solution::capitalize_first_char(s.to_string()))
            .collect::<Vec<String>>()
            .join(" ")
    }

    fn capitalize_first_char(s: String) -> String {
        s.chars()
            .enumerate()
            .map(|(i, c)| {
                if i == 0 && s.len() > 2 {
                    c.to_uppercase().to_string()
                } else {
                    c.to_lowercase().to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("")
    }
}
