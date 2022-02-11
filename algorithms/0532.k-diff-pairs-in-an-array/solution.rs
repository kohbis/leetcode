use std::collections::HashMap;

impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for &num in &nums {
            let entry = map.entry(num).or_insert(0);
            *entry += 1;
        }

        let mut ans = 0i32;
        for (&key, &value) in map.iter() {
            if map.get(&(key + k)).is_some() {
                if (k == 0 && value > 1) || k > 0 {
                    ans += 1;
                }
            }
        }

        ans
    }
}
