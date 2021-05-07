impl Solution {
    pub fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
        (0..nums.len())
            .collect::<Vec<_>>()
            .into_iter()
            .filter(|&i| nums[i] == target)
            .into_iter()
            .map(|n| (n as i32 - start).abs())
            .into_iter()
            .min()
            .unwrap() as i32
    }
}
