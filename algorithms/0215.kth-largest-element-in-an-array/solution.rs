use std::collections::HashMap;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut counter: HashMap<i32, i32> = HashMap::new();
        for n in nums {
            *counter.entry(n).or_insert(0) += 1;
        }

        let mut k = k;
        let mut keys: Vec<i32> = counter.keys().cloned().collect();
        keys.sort_unstable_by(|a, b| b.cmp(a));
        for key in keys {
            k -= counter[&key];
            if k <= 0 {
                return key;
            }
        }

        -1 // unreachable!
    }
}
