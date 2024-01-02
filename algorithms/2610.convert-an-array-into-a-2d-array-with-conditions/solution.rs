use std::collections::HashMap;

impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for num in nums.into_iter() {
            *map.entry(num).or_insert(0) += 1;
        }
        let max_size = map.values().max().unwrap().clone() as usize;

        let mut res: Vec<Vec<i32>> = vec![vec![]; max_size];
        for (num, count) in map {
            for i in 0..count {
                res[i as usize].push(num);
            }
        }

        return res;
    }
}
