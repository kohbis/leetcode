use std::collections::HashSet;

impl Solution {
    pub fn find_subarrays(nums: Vec<i32>) -> bool {
        nums.windows(2)
            .map(|w| w[0] + w[1])
            .collect::<HashSet<i32>>()
            .len()
            < nums.len() - 1
    }
}
