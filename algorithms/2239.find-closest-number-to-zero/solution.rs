impl Solution {
    pub fn find_closest_number(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        nums.reverse();
        nums.into_iter().min_by_key(|x| x.abs()).unwrap()
    }
}
