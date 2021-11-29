impl Solution {
    pub fn max_distance(colors: Vec<i32>) -> i32 {
        let len = colors.len();

        let mut left = 0usize;
        while colors[left] == colors[len - 1] {
            left += 1;
        }

        let mut right = len - 1;
        while colors[0] == colors[right] {
            right -= 1;
        }

        (len - 1 - left).max(right) as _
    }
}
