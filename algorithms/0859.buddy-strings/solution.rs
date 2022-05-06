use std::collections::HashMap;

impl Solution {
    pub fn buddy_strings(s: String, goal: String) -> bool {
        let len = s.len();

        if s.len() != goal.len() || s.len() < 2 {
            return false;
        }

        let s_chars: Vec<char> = s.chars().collect();
        let goal_chars: Vec<char> = goal.chars().collect();

        let mut count = 0;
        let mut indexes = vec![];
        for i in 0..len {
            if s_chars[i] != goal_chars[i] {
                count += 1;
                indexes.push(i);
            }

            if count > 2 {
                return false;
            }
        }

        match count {
            2 => {
                s_chars[indexes[0]] == goal_chars[indexes[1]]
                    && s_chars[indexes[1]] == goal_chars[indexes[0]]
            }
            0 => {
                let mut hm: HashMap<char, i32> = HashMap::new();
                for c in s.chars() {
                    let count = hm.entry(c).or_default();
                    *count += 1;

                    if *count >= 2 {
                        return true;
                    }
                }
                false
            }
            _ => false,
        }
    }
}
