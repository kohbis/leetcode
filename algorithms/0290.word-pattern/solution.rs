impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        use std::collections::HashMap;
        let mut map = HashMap::new();

        let words = s.split_whitespace().collect::<Vec<&str>>();
        let chars = pattern.chars().collect::<Vec<char>>();

        if words.len() != chars.len() {
            return false;
        }

        for (i, c) in chars.iter().enumerate() {
            if map.contains_key(&c) {
                if map[c] != words[i] {
                    return false;
                }
            } else {
                map.insert(c, words[i]);
            }
        }

        let mut values = map.values().cloned().collect::<Vec<&str>>();
        values.sort_unstable();
        values.dedup();

        map.values().len() == values.len()
    }
}
