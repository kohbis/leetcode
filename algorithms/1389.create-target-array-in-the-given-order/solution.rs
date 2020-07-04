impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::with_capacity(nums.len());
        for i in 0..nums.len() {
            res.insert(index[i] as usize, nums[i]);
        }
        res
    }
}
