impl Solution {
    pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
        use std::cmp;
        use std::collections::HashMap;

        let mut map: HashMap<i32, i32> = HashMap::new();

        for num in low_limit..=high_limit {
            let sum = Self::digits_sum(num);
            let count = map.entry(sum).or_insert(0);
            *count += 1
        }

        let mut max = 0;
        for (_, v) in map {
            max = cmp::max(v, max)
        }
        max
    }

    fn digits_sum(num: i32) -> i32 {
        num.to_string().chars().map(|c| c as i32 - 48).sum()
    }
}
