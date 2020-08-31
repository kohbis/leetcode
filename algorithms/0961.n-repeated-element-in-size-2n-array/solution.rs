use std::collections::HashSet;

impl Solution {
    pub fn repeated_n_times(a: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut h = HashSet::new();

        for i in a {
            if h.contains(&i) {
                res = i;
                break;
            }
            h.insert(i);
        }

        res
    }
}
