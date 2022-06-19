use std::collections::HashMap;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut res = 0i32;
        let mut map: HashMap<i32, i32> = HashMap::new();
        for n in nums {
            *map.entry(n).or_default() += 1;

            if map.get(&n) > map.get(&res) {
                res = n;
            }
        }

        res
    }
}
