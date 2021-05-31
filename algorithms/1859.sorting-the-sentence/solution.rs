impl Solution {
    pub fn sort_sentence(s: String) -> String {
        let mut words = s
            .split_whitespace()
            .map(|word| {
                let mut letters = word.chars().collect::<Vec<char>>();
                let idx = letters.pop().unwrap();
                (idx, letters.iter().collect::<String>())
            })
            .collect::<Vec<(char, String)>>();

        words.sort_by(|a, b| a.0.cmp(&b.0));
        words
            .into_iter()
            .map(|x| x.1)
            .collect::<Vec<String>>()
            .join(" ")
    }
}
