use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn finding_users_active_minutes(logs: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut map: HashMap<i32, HashSet<i32>> = HashMap::new();

        for log in logs {
            let times = map.entry(log[0]).or_insert(HashSet::new());
            times.insert(log[1]);
        }

        let mut res = vec![0; k as usize];
        for (_, times) in map {
            res[times.len() - 1] += 1;
        }
        res
    }
}
