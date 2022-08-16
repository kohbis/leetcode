impl Solution {
    pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
        let mut res = 0;
        for i in 0..nums.len() - 2 {
            for j in i + 1..nums.len() - 1 {
                for k in j + 1..nums.len() {
                    if i < j && j < k && nums[j] - nums[i] == diff && nums[k] - nums[j] == diff {
                        res += 1;
                    }
                }
            }
        }
        res
    }
}
