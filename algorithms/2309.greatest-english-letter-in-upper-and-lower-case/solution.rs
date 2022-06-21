use std::collections::HashSet;

impl Solution {
    pub fn greatest_letter(s: String) -> String {
        let mut res = "".to_string();
        let mut hs: HashSet<char> = HashSet::new();
        for c in s.chars() {
            if (c.is_uppercase() && hs.contains(&c.to_ascii_lowercase()))
                || (c.is_lowercase() && hs.contains(&c.to_ascii_uppercase()))
            {
                let letter = c.to_ascii_uppercase().to_string();
                if res < letter {
                    res = letter;
                }
            }

            hs.insert(c);
        }

        res
    }
}
