impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let len = nums.len();
        let mut nums = nums;
        nums.sort_unstable();

        let mut closest = i32::MAX;
        for i in 0..len {
            let mut j = i + 1;
            let mut k = len - 1;

            while j < k {
                let sum = nums[i] + nums[j] + nums[k];
                if sum == target {
                    return target;
                } else if sum > target {
                    k -= 1;
                } else {
                    j += 1;
                }

                if (sum - target).abs() < (closest - target).abs() {
                    closest = sum;
                }
            }
        }
        return closest;
    }
}
