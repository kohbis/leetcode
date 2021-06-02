use std::cmp;

impl Solution {
    pub fn min_pair_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        let mut res = 0;
        let len = nums.len();
        for i in 0..(len / 2) {
            res = cmp::max(res, nums[i] + nums[len - i - 1]);
        }

        res
    }
}
