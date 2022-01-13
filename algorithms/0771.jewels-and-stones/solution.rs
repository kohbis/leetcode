use std::collections::HashMap;

impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let mut stone_count: HashMap<char, i32> = HashMap::new();
        for s in stones.chars() {
            let entry = stone_count.entry(s).or_insert(0);
            *entry += 1;
        }

        jewels.chars().fold(0i32, |sum, c| {
            if let Some(n) = stone_count.get(&c) {
                sum + n
            } else {
                sum
            }
        })
    }
}
