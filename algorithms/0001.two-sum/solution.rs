use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());
        for (idx, n) in nums.iter().enumerate() {
            let diff = target - n;
            if let Some(&i) = map.get(&diff) {
                return vec![i as i32, idx as i32];
            } else {
                map.insert(n, idx);
            }
        }
        vec![]
    }
}
