impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        let mut max: i32 = 0;
        for a in accounts {
            let mut sum: i32 = 0;
            for n in a {
                sum += n
            }

            if sum > max {
                max = sum
            }
        }

        max
    }
}
