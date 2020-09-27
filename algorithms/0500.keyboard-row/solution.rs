impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let rows = ["qwertyuiop", "asdfghjkl", "zxcvbnm"];

        words
            .into_iter()
            .filter(|word| {
                rows
                    .iter()
                    .fold(false, |same_row, row| {
                        same_row || word.to_lowercase().chars().all(|c| row.contains(c))
                    })
            })
            .collect()
    }
}
