impl Solution {
    pub fn has_trailing_zeros(nums: Vec<i32>) -> bool {
        let mut count = 0;
        for num in nums {
            count += (num + 1) % 2;
        }
        count >= 2
    }
}
