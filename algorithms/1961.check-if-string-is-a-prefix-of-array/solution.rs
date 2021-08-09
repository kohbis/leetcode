impl Solution {
    pub fn is_prefix_string(s: String, words: Vec<String>) -> bool {
        let mut curr = String::new();

        for word in words {
            curr.push_str(&word);

            if s == curr {
                return true;
            }

            if s.len() <= curr.len() {
                return false;
            }
        }

        false
    }
}
