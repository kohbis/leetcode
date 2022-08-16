use std::collections::HashMap;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        // char => [first_index, count]
        let mut hm: HashMap<char, (i32, i32)> = HashMap::new();
        for (i, c) in s.chars().enumerate() {
            let entry = hm.entry(c).or_insert((i as i32, 0));
            entry.1 += 1;
        }

        let mut uniq: Vec<i32> = hm
            .iter()
            .filter(|&(k, v)| v.1 == 1)
            .map(|(k, v)| v.0)
            .collect();
        match uniq.into_iter().min() {
            Some(res) => res,
            None => -1,
        }
    }
}
