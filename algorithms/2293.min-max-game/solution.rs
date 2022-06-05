use std::cmp::{max, min};

impl Solution {
    pub fn min_max_game(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut mid = nums.len() / 2;

        while mid > 0 {
            for i in 0..mid {
                if i % 2 == 0 {
                    nums[i] = min(nums[i * 2], nums[i * 2 + 1]);
                } else {
                    nums[i] = max(nums[i * 2], nums[i * 2 + 1]);
                }
            }

            mid /= 2;
        }

        nums[0]
    }
}
