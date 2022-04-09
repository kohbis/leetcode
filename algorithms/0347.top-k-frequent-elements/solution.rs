use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut count: HashMap<i32, i32> = HashMap::new();
        for n in nums {
            let entry = count.entry(n).or_insert(0);
            *entry += 1;
        }

        let mut count_vec: Vec<(&i32, &i32)> = count.iter().collect();
        count_vec.sort_unstable_by(|a, b| b.1.cmp(a.1));
        count_vec.iter().map(|x| *(x.0)).take(k as usize).collect()
    }
}
