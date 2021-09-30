impl Solution {
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        let mut max = 0;

        for i in 0..(nums.len() - 1) {
            for j in (i + 1)..nums.len() {
                if nums[j] - nums[i] > max {
                    max = nums[j] - nums[i];
                }
            }
        }

        if max > 0 {
            max
        } else {
            -1
        }
    }
}
