impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let (mut max, mut second_max) = (0, 0);
        for num in nums.iter() {
            if *num > max {
                second_max = max;
                max = *num;
            } else if *num > second_max {
                second_max = *num;
            }
        }
        (max - 1) * (second_max - 1)
    }
}
