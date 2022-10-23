use std::collections::HashSet;

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut hs: HashSet<i32> = HashSet::new();
        let mut res: Vec<i32> = Vec::with_capacity(2);
        for num in &nums {
            if !hs.insert(*num) {
                res.push(*num);
            }
        }
        for i in 1..=nums.len() as i32 {
            if !hs.contains(&i) {
                res.push(i);
                break;
            }
        }
        res
    }
}
