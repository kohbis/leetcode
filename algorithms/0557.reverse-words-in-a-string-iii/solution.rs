impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut res = String::new();
        let words = s.split_whitespace().collect::<Vec<&str>>();

        for word in words {
            res.push_str(&word.chars().rev().collect::<String>());
            res.push(' ');
        }

        res.trim_end().to_string()
    }
}
