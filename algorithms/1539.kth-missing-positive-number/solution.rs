use std::collections::HashSet;

impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        let mut nums: HashSet<&i32> = arr.iter().collect();
        let mut i: i32 = 1;
        let mut res: i32 = 1;

        let mut k = k;
        while k > 0 {
            if !nums.contains(&i) {
                res = i;
                k -= 1;
            }
            i += 1;
        }

        res
    }
}
