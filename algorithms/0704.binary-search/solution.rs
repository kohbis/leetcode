impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0 as i32;
        let mut right = (nums.len() - 1) as i32;

        while left <= right {
            let mid = ((left + right) / 2) as usize;

            if target == nums[mid] {
                return mid as i32;
            } else if target < nums[mid] {
                right = mid as i32 - 1;
            } else {
                left = mid as i32 + 1;
            }
        }

        -1
    }
}
