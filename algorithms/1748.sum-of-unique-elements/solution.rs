impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let mut sum: i32 = 0;
        let mut map: HashMap<i32, i32> = HashMap::new();

        for n in nums {
            let value = map.entry(n).or_insert(0);
            *value += 1
        }

        for (k, v) in map {
            if v == 1 {
                sum += k
            }
        }

        sum
    }
}
