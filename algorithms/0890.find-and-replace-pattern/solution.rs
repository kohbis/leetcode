use std::collections::HashMap;

impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        let base: Vec<i32> = Solution::pattern_to_integers(&pattern);

        let mut res = vec![];
        for word in words {
            if base == Solution::pattern_to_integers(&word) {
                res.push(word);
            }
        }
        res
    }

    fn pattern_to_integers(word: &str) -> Vec<i32> {
        let mut map: HashMap<char, i32> = HashMap::new();

        let mut pattern: Vec<i32> = vec![];
        for c in word.chars() {
            match map.get(&c) {
                Some(_) => {
                    pattern.push(map[&c]);
                }
                None => {
                    let next = map.len() as i32;
                    pattern.push(next);
                    map.insert(c, next);
                }
            }
        }
        pattern
    }
}
