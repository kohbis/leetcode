impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        // sort & dedup
        // let mut letters: Vec<char> = sentence.chars().collect();
        // letters.sort_unstable();
        // letters.dedup();
        // letters.len() == 26

        // using Hash
        use std::collections::HashMap;
        let mut map: HashMap<char, i32> = HashMap::new();
        for c in sentence.chars() {
            let count = map.entry(c).or_insert(0);
            *count += 1
        }
        map.keys().len() == 26
    }
}
