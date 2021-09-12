impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut res: i32 = 0;
        for i in 0..nums.len() {
            for j in i..nums.len() {
                if nums[i] == nums[j] && i < j {
                    res += 1;
                }
            }
        }

        res
    }
}
