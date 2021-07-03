impl Solution {
    pub fn max_product_difference(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut nums = nums;
        nums.sort_unstable();

        nums[len - 1] * nums[len - 2] - nums[0] * nums[1]
    }
}
