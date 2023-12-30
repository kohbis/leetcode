impl Solution {
    pub fn number_game(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_unstable();

        let mut arr: Vec<i32> = Vec::with_capacity(nums.len());
        for i in (0..nums.len()).step_by(2) {
            arr.push(nums[i + 1]);
            arr.push(nums[i]);
        }
        arr
    }
}
