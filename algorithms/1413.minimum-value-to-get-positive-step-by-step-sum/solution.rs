impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let mut min = i32::MAX;

        nums.iter().fold(0, |mut sum, n| {
            sum += n;
            min = min.min(sum);

            sum
        });

        1.max(min * -1 + 1)
    }
}
