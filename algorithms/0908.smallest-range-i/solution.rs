impl Solution {
    pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
        // (max - k) - (min - k)
        std::cmp::max(
            0,
            nums.iter().max().unwrap() - nums.iter().min().unwrap() - 2 * k,
        )
    }
}
