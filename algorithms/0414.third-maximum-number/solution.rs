impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut nums: Vec<i32> = nums.clone();
        nums.sort_unstable();
        nums.dedup();

        let len = nums.len();
        if len < 3 {
            nums[len - 1]
        } else {
            nums[len - 3]
        }
    }
}
