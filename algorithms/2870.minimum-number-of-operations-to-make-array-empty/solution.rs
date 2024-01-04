use std::collections::HashMap;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut count: HashMap<i32, i32> = HashMap::new();
        for n in nums {
            *count.entry(n).or_default() += 1;
        }

        let mut res = 0;
        let values = count.into_values().collect::<Vec<_>>();
        for v in values {
            if v == 1 {
                return -1;
            }

            res += v / 3;
            if v % 3 != 0 {
                res += 1;
            }
        }

        return res;
    }
}
