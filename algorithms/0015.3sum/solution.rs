use std::collections::HashSet;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        }

        let mut nums = nums;
        nums.sort_unstable();

        let mut set: HashSet<Vec<i32>> = HashSet::new();
        for i in 0..(nums.len() - 2) {
            let mut left = i + 1;
            let mut right = nums.len() - 1;

            while left < right {
                let sum = nums[i] + nums[left] + nums[right];

                if sum == 0 {
                    set.insert(vec![nums[i], nums[left], nums[right]]);
                    left += 1;
                    right -= 1;
                } else if sum < 0 {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }

        set.iter().cloned().collect()
    }
}
