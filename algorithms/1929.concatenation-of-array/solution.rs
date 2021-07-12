impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let l = nums.len();
        let mut res = vec![0; l * 2];
        for i in 0..l {
            res[i] = nums[i];
            res[i + l] = nums[i];
        }

        res
    }
}
