use std::collections::HashMap;

impl Solution {
    pub fn find_lonely(nums: Vec<i32>) -> Vec<i32> {
        let mut hash: HashMap<i32, i32> = HashMap::new();

        for &num in &nums {
            let entry = hash.entry(num).or_default();
            *entry += 1;
        }

        let mut res: Vec<i32> = vec![];
        for num in nums {
            if let Some(value) = hash.get(&num) {
                if *value == 1 {
                    if !hash.contains_key(&(num + 1)) && !hash.contains_key(&(num - 1)) {
                        res.push(num);
                    }
                }
            }
        }

        res
    }
}
