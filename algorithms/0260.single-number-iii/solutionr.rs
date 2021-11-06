use std::collections::HashMap;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for n in nums {
            if let Some(x) = map.get(&n) {
                map.remove(&n);
            } else {
                map.insert(n, n);
            }
        }

        map.values().cloned().collect()
    }
}
