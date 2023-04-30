impl Solution {
    pub fn maximize_sum(nums: Vec<i32>, k: i32) -> i32 {
        let m = nums.iter().max().unwrap();
        k * m + k * (k - 1) / 2
    }
}
