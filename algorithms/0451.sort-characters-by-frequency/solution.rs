use std::collections::HashMap;

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut map: HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
            let entry = map.entry(c).or_insert(0);
            *entry += 1;
        }

        let mut counter: Vec<(char, i32)> = map.into_iter().map(|(c, n)| (c, n)).collect();
        counter.sort_by(|a, b| b.1.cmp(&a.1));

        let mut res: Vec<char> = vec![];
        for (c, n) in counter {
            for _ in 0..n as usize {
                res.push(c);
            }
        }

        res.iter().collect()
    }
}
