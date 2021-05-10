impl Solution {
    pub fn min_subsequence(nums: Vec<i32>) -> Vec<i32> {
        let mut sub = Vec::new();
        let mut sub_sum = 0;

        let mut sum = nums.iter().sum();
        let mut nums = nums.clone();
        nums.sort();
        nums.reverse();

        while sub_sum <= sum {
            sub.push(nums[0]);

            sum -= nums[0];
            sub_sum += nums[0];

            nums.remove(0);
        }

        sub
    }
}
