impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        if let Some(idx) = word.chars().into_iter().position(|x| x == ch) {
            return format!(
                "{}{}",
                &word[0..=idx].chars().rev().collect::<String>(),
                &word[idx + 1..]
            );
        } else {
            return word;
        }
    }
}
