impl Solution {
    pub fn most_words_found(sentences: Vec<String>) -> i32 {
        let mut max = 0i32;
        for s in sentences {
            let count = s.split_whitespace().collect::<Vec<&str>>().len();
            max = max.max(count as i32);
        }

        max
    }
}
