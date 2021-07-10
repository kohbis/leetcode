impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp = vec![1; nums.len()];
        let mut longest = 1;
        for i in 0..nums.len() {
            for j in 0..i {
                if nums[j] < nums[i] {
                    if dp[i] < dp[j] + 1 {
                        dp[i] = dp[j] + 1
                    }

                    if longest < dp[i] {
                        longest = dp[i]
                    }
                }
            }
        }

        longest
    }
}
