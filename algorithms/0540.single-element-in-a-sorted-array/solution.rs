use std::collections::HashSet;

impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let mut set: HashSet<i32> = HashSet::new();
        for n in nums {
            if !set.insert(n) {
                set.remove(&n);
            }
        }
        // unreachable -1
        set.into_iter().next().unwrap_or(-1)
    }
}
