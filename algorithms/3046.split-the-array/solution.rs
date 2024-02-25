use std::collections::HashMap;

impl Solution {
    pub fn is_possible_to_split(nums: Vec<i32>) -> bool {
        let mut count: HashMap<i32, i32> = HashMap::new();
        for n in nums {
            let x = count.entry(n).or_insert(0);
            *x += 1;
            if *x > 2 {
                return false;
            }
        }
        true
    }
}
