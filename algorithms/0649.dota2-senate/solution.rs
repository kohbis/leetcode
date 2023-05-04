use std::collections::VecDeque;

impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let mut radiant: VecDeque<usize> = VecDeque::new();
        let mut dire: VecDeque<usize> = VecDeque::new();

        for (i, c) in senate.chars().enumerate() {
            if c == 'R' {
                radiant.push_back(i);
            } else {
                dire.push_back(i);
            }
        }

        let n = senate.len();
        while !radiant.is_empty() && !dire.is_empty() {
            match (radiant.pop_front(), dire.pop_front()) {
                (Some(r), Some(d)) => {
                    if r < d {
                        radiant.push_back(r + n);
                    } else {
                        dire.push_back(d + n);
                    }
                }
                (_, _) => {}
            }
        }

        if radiant.is_empty() {
            "Dire".to_string()
        } else {
            "Radiant".to_string()
        }
    }
}
