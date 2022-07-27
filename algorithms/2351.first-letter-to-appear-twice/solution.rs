use std::collections::HashSet;

impl Solution {
    pub fn repeated_character(s: String) -> char {
        let mut hs: HashSet<char> = HashSet::new();
        for c in s.chars() {
            if hs.contains(&c) {
                return c;
            } else {
                hs.insert(c);
            }
        }
        unreachable!()
    }
}
