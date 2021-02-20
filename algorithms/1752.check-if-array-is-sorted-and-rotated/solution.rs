impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let len: usize = nums.len();
        let mut rotate: bool = false;
        let mut max: i32 = 0;

        for i in 0..len {
            let left = if i == 0 { nums[len - 1] } else { nums[i - 1] };

            if nums[i] >= left {
                if rotate && nums[i] > max {
                    return false;
                }
            } else {
                if rotate {
                    return false;
                }
                rotate = true;
                max = left
            }
        }

        true
    }
}
