use std::collections::HashMap;

impl Solution {
    pub fn most_frequent(nums: Vec<i32>, key: i32) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for i in 0..nums.len() - 1 {
            if nums[i] == key {
                *map.entry(nums[i + 1]).or_default() += 1;
            }
        }

        let (mut res, mut curr) = (0, 0);
        for (k, v) in map {
            if curr < v {
                curr = v;
                res = k;
            }
        }

        res
    }
}
