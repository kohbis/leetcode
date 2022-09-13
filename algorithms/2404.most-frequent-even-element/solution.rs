use std::collections::BTreeMap;

impl Solution {
    pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
        let mut curr_max = -1;
        let mut count: BTreeMap<i32, i32> = BTreeMap::new();
        for n in nums {
            if n > 0 && n % 2 == 0 {
                let e = count.entry(n).or_insert(0);
                *e += 1;

                curr_max = curr_max.max(*e);
            }
        }

        for (k, v) in count {
            if v == curr_max {
                return k;
            }
        }

        -1
    }
}
