impl Solution {
    pub fn sort_sentence(s: String) -> String {
        let mut words = s
            .split_whitespace()
            .map(|word| {
                (
                    word[word.len() - 1..].parse::<i32>().unwrap(),
                    word[..word.len() - 1].to_string(),
                )
            })
            .collect::<Vec<(i32, String)>>();

        words.sort_by(|a, b| a.0.cmp(&b.0));
        words
            .into_iter()
            .map(|x| x.1)
            .collect::<Vec<String>>()
            .join(" ")
    }
}
