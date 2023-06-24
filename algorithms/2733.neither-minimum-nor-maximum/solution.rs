impl Solution {
    pub fn find_non_min_or_max(nums: Vec<i32>) -> i32 {
        let (mut max, mut min) = (0, 100);
        for i in 0..nums.len() {
            max = max.max(nums[i]);
            min = min.min(nums[i]);
        }
        for n in nums {
            if n != max && n != min {
                return n;
            }
        }
        -1
    }
}
