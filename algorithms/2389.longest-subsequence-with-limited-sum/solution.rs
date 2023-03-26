impl Solution {
    pub fn answer_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_unstable();

        for i in 0..nums.len() - 1 {
            nums[i + 1] += nums[i];
        }

        queries
            .iter()
            .map(|query| match nums.binary_search(query) {
                Ok(i) => i as i32 + 1,
                Err(i) => i as i32,
            })
            .collect()
    }
}
