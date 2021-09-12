impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        nums.iter()
            .map(|n| nums.iter().filter(|&x| n > x).count() as i32)
            .collect()
    }
}
