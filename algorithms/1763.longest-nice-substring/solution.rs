use std::collections::HashSet;

impl Solution {
    pub fn longest_nice_substring(s: String) -> String {
        if s.len() < 2 {
            return "".to_string();
        }

        let mut uniqs: HashSet<char> = HashSet::new();
        for c in s.chars() {
            uniqs.insert(c);
        }

        for (i, c) in s.chars().into_iter().enumerate() {
            if uniqs.contains(&c.to_ascii_lowercase()) && uniqs.contains(&c.to_ascii_uppercase()) {
                continue;
            }

            let left = Self::longest_nice_substring(String::from(&s[0..i]));
            let right = Self::longest_nice_substring(String::from(&s[i + 1..]));

            if left.len() >= right.len() {
                return left;
            } else {
                return right;
            }
        }

        s
    }
}
