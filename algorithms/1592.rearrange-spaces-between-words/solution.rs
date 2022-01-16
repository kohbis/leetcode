impl Solution {
    pub fn reorder_spaces(text: String) -> String {
        let words: Vec<&str> = text.split_whitespace().collect();
        let space_count = text.len() - words.iter().fold(0usize, |sum, s| sum + s.len());

        let (separator, extra) = if space_count > 0 && words.len() > 1 {
            (
                " ".repeat(space_count / (words.len() - 1)),
                " ".repeat(space_count % (words.len() - 1)),
            )
        } else {
            ("".to_string(), " ".repeat(space_count))
        };

        format!("{}{}", words.join(&separator), extra)
    }
}
