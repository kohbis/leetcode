use std::cmp::min;

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut nums = nums;
        nums.sort_unstable();

        let mut res = i32::MAX;

        for i in (k - 1)..nums.len() {
            res = min(res, nums[i] - nums[i - k + 1]);
        }

        res
    }
}
