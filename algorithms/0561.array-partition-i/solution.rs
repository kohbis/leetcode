impl Solution {
    pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
        let mut res: i32 = 0;
        let mut nums: Vec<i32> = nums.clone();
        nums.sort();

        for (idx, num) in nums.iter().enumerate() {
            if idx % 2 == 0 {
                res += num;
            }
        }

        res
    }
}
