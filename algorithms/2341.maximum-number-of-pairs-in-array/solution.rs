use std::collections::HashMap;

impl Solution {
    pub fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
        let mut count: HashMap<i32, i32> = HashMap::new();
        for n in nums {
            *count.entry(n).or_insert(0) += 1;
        }

        count.values().fold(vec![0, 0], |mut acc, x| {
            acc[0] += x / 2;
            acc[1] += x % 2;
            acc
        })
    }
}
