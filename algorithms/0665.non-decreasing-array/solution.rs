impl Solution {
    pub fn check_possibility(nums: Vec<i32>) -> bool {
        let size = nums.len();
        let mut nums = nums;
        let mut count = 0;

        for i in 1..size {
            if nums[i - 1] > nums[i] {
                count += 1;
                if count > 1 {
                    return false;
                }

                if i < 2 || nums[i - 2] <= nums[i] {
                    nums[i - 1] = nums[i];
                } else {
                    nums[i] = nums[i - 1];
                }
            }
        }

        true
    }
}
