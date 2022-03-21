use std::collections::HashSet;

impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        let mut hs: HashSet<i32> = HashSet::new();
        for n in nums {
            if let Some(_) = hs.get(&n) {
                hs.remove(&n);
            } else {
                hs.insert(n);
            }
        }

        hs.len() == 0
    }
}
