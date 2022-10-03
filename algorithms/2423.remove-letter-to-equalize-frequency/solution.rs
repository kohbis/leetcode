use std::collections::{BTreeMap, HashMap};

impl Solution {
    pub fn equal_frequency(word: String) -> bool {
        let mut char_counts: HashMap<char, i32> = HashMap::new();
        for c in word.chars() {
            let e = char_counts.entry(c).or_insert(0);
            *e += 1;
        }

        let mut value_counts: BTreeMap<i32, i32> = BTreeMap::new();
        for v in char_counts.values().cloned() {
            let e = value_counts.entry(v).or_insert(0);
            *e += 1;
        }
        match value_counts.len() {
            1 => {
                if char_counts.len() == 2 {
                    return value_counts.contains_key(&1);
                }
                true
            }
            2 => {
                if let Some(count) = value_counts.get(&1) {
                    if count == &1 {
                        return true;
                    }
                }

                let keys: Vec<i32> = value_counts.keys().cloned().collect();
                let values: Vec<i32> = value_counts.values().cloned().collect();
                return keys[1] - keys[0] == 1 && (values[0] == 1 || values[1] == 1);
            }
            _ => false,
        }
    }
}
