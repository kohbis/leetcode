impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        let neg = nums.partition_point(|&x| x < 0) as i32;
        let pos = (nums.len() - nums.partition_point(|&x| x < 1)) as i32;
        neg.max(pos)
    }
}
