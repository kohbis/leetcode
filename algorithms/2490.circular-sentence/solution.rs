impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let mut words: Vec<&str> = sentence.split_whitespace().collect();
        words.push(words[0].clone());
        for w in words.windows(2) {
            let (a, b) = (w[0], w[1]);
            if a.chars().nth(a.len() - 1) != b.chars().nth(0) {
                return false;
            }
        }
        true
    }
}
