use std::collections::BTreeSet;

impl Solution {
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        let mut pos: BTreeSet<i32> = BTreeSet::new();
        let mut neg: BTreeSet<i32> = BTreeSet::new();

        for n in nums {
            if 0 < n {
                pos.insert(n);
            } else {
                neg.insert(n);
            }
        }

        for n in pos.into_iter().rev() {
            if neg.contains(&(-n)) {
                return n;
            }
        }

        -1
    }
}
