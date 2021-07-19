use std::collections::HashSet;

impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let mut count = 0 as i32;
        let broken_chars = &broken_letters.chars().collect::<Vec<char>>();

        'outer: for word in text.split_whitespace() {
            let set = word.chars().collect::<HashSet<char>>();
            for bc in broken_chars {
                if set.contains(&bc) {
                    continue 'outer;
                }
            }

            count += 1;
        }
        count
    }
}
