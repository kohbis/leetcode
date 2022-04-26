use std::collections::HashMap;

impl Solution {
    pub fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let nums_len = nums.len();
        let mut count: HashMap<i32, i32> = HashMap::new();

        for digits in nums {
            for d in digits {
                *count.entry(d).or_default() += 1
            }
        }

        let mut ans: Vec<i32> = vec![];
        for (k, v) in count {
            if v as usize == nums_len {
                ans.push(k);
            }
        }

        ans.sort_unstable();
        ans
    }
}
