impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        use std::collections::HashMap;
        let mut map = HashMap::new();

        if s.len() != t.len() {
            return false
        }

        let s_chars = s.chars().collect::<Vec<char>>();
        let t_chars = t.chars().collect::<Vec<char>>();

        for (i, c) in s_chars.iter().enumerate() {
            if map.contains_key(&c) {
                if map[c] != t_chars[i] {
                    return false
                }
            } else {
                map.insert(c, t_chars[i]);
            }
        }

        let mut values = map.values().cloned().collect::<Vec<char>>();
        values.sort_unstable();
        values.dedup();

        map.values().len() == values.len()
    }
}

