use std::collections::HashMap;

impl Solution {
    pub fn check_distances(s: String, distance: Vec<i32>) -> bool {
        let mut positions: HashMap<char, Vec<i32>> = HashMap::new();
        for (i, c) in s.chars().enumerate() {
            let entry = positions.entry(c).or_insert(vec![]);
            (*entry).push(i as i32);
        }

        for (c, p) in positions {
            let i = c as usize - 97;
            if distance[i] != p[1] - p[0] - 1 {
                return false;
            }
        }

        true
    }
}
