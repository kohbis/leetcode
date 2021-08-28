impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        let (mut low, mut high) = (low, high);
        let mut sum = (high - low) / 2;

        if (high - low) % 2 == 1 {
            sum += 1;
        } else if low % 2 == 1 {
            sum += 1;
        }

        sum
    }
}
