use std::collections::HashMap;

impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        fn count_letters(chars: &str) -> HashMap<char, i32> {
            let mut map: HashMap<char, i32> = HashMap::new();
            for c in chars.chars() {
                *map.entry(c).or_insert(0) += 1;
            }
            map
        }

        let chars_counts = count_letters(&chars);
        let mut res = 0;
        'outer: for word in words {
            let word_counts = count_letters(&word);
            for (letter, count) in word_counts {
                if chars_counts.contains_key(&letter) {
                    if chars_counts[&letter] >= count {
                        continue;
                    }
                }
                continue 'outer;
            }
            res += word.len() as i32;
        }

        res
    }
}
