impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        for w in arr.windows(3) {
            if [w[0], w[1], w[2]].iter().filter(|&i| i % 2 == 0).count() == 0 {
                return true;
            }
        }
        false
    }
}
