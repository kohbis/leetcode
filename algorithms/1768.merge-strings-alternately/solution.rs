use std::cmp;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut res = String::new();
        let len = cmp::max(word1.len(), word2.len());

        let chars1: Vec<char> = word1.chars().collect();
        let chars2: Vec<char> = word2.chars().collect();

        for i in 0..len {
            if i < chars1.len() {
                res.push(chars1[i]);
            }
            if i < chars2.len() {
                res.push(chars2[i]);
            }
        }

        res
    }
}
