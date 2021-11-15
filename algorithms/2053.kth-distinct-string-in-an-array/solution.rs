use std::collections::HashMap;

impl Solution {
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        let mut hm: HashMap<String, Vec<i32>> = HashMap::new();

        for (i, s) in arr.into_iter().enumerate() {
            let entry = hm.entry(s).or_insert(vec![0, i as i32]);
            (*entry)[0] += 1;
        }

        let mut counts: Vec<(&String, i32)> = hm
            .iter()
            .filter(|(_, v)| v[0] == 1)
            .map(|(k, v)| (k, v[1]))
            .collect();
        counts.sort_by(|a, b| a.1.cmp(&b.1));

        if counts.len() >= k as usize {
            counts[k as usize - 1].0.to_string()
        } else {
            "".to_string()
        }
    }
}
