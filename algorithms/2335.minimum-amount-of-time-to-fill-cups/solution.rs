use std::cmp::max;

impl Solution {
    pub fn fill_cups(amount: Vec<i32>) -> i32 {
        let max_amount: i32 = *amount.iter().max().unwrap();
        let sum: i32 = amount.iter().sum();

        max(max_amount, (sum + 1) / 2)
    }
}

