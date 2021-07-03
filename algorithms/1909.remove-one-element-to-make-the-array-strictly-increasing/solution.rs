impl Solution {
    pub fn can_be_increasing(nums: Vec<i32>) -> bool {
        let mut pre = nums[0];
        let mut removed = false;

        for i in 1..nums.len() {
            if pre < nums[i] {
                pre = nums[i];
            } else {
                if removed {
                    return false;
                }
                removed = true;

                if i == 1 || nums[i - 2] < nums[i] {
                    pre = nums[i];
                }
            }
        }

        true
    }
}
