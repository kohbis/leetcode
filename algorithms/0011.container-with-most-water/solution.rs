impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max = 0i32;
        let (mut left, mut right) = (0, height.len() - 1);

        while left < right {
            let amount = height[left].min(height[right]) * (right - left) as i32;

            max = max.max(amount);

            if height[left] > height[right] {
                right -= 1;
            } else {
                left += 1;
            }
        }

        max
    }
}
