use std::collections::HashSet;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let hs: HashSet<i32> = nums.into_iter().collect();
        let size = hs.len() as i32;
        if hs.contains(&0) {
            size - 1
        } else {
            size
        }
    }
}
