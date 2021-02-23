impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        use std::collections::VecDeque;

        let mut chars1: VecDeque<char> = word1.chars().collect();
        let mut chars2: VecDeque<char> = word2.chars().collect();

        let mut res: String = "".to_string();
        while chars1.len() > 0 || chars2.len() > 0 {
            if chars1.len() > 0 {
                res.push_str(&(chars1.pop_front().unwrap().to_string()))
            }

            if chars2.len() > 0 {
                res.push_str(&(chars2.pop_front().unwrap().to_string()))
            }
        }

        res
    }
}
