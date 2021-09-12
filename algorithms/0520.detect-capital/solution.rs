impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let v_word: Vec<char> = word.chars().collect();
        let uppers = v_word
            .iter()
            .filter(|&&c| c.is_uppercase())
            .cloned()
            .collect::<Vec<char>>()
            .len();
        uppers == 0 || uppers == word.len() || (uppers == 1 && v_word[0].is_uppercase())
    }
}
