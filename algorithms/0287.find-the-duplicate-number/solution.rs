use std::collections::HashSet;

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut hs: HashSet<i32> = HashSet::new();
        for num in nums {
            if hs.contains(&num) {
                return num;
            } else {
                hs.insert(num);
            }
        }

        unreachable!()
    }
}
