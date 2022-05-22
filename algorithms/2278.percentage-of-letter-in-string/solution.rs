use std::collections::HashMap;

impl Solution {
    pub fn percentage_letter(s: String, letter: char) -> i32 {
        let mut map: HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
            *map.entry(c).or_default() += 1
        }

        if let Some(count) = map.get(&letter) {
            100 * count / s.len() as i32
        } else {
            0
        }
    }
}
