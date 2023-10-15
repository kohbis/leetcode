impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let mut res: i64 = 0;
        for i in 0..nums.len() - 2 {
            for j in i + 1..nums.len() - 1 {
                for k in j + 1..nums.len() {
                    res = res.max(((nums[i] - nums[j]) as i64 * nums[k] as i64));
                }
            }
        }
        res
    }
}
