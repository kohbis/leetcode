impl Solution {
    pub fn largest_perimeter(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable_by(|a, b| b.cmp(a));
        for w in nums.windows(3) {
            if w[0] < w[1] + w[2] {
                return w.iter().sum();
            }
        }
        0
    }
}
