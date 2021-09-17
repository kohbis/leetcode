impl Solution {
    pub fn find_middle_index(nums: Vec<i32>) -> i32 {
        let sum = nums.iter().sum::<i32>();
        let (mut left, mut right) = (0, sum);

        for i in 0..nums.len() {
            right -= nums[i];

            if left == right {
                return i as i32;
            }

            left += nums[i];
        }

        -1
    }
}
