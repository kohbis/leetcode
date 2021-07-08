impl Solution {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len();
        let m = nums2.len();
        let mut dp = vec![vec![0; m + 1]; n + 1];

        let mut max = 0;
        for i in 1..=n {
            for j in 1..=m {
                if nums1[i - 1] == nums2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                }

                if max < dp[i][j] {
                    max = dp[i][j];
                }
            }
        }

        max
    }
}
