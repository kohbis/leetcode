use std::collections::{BTreeMap, HashMap};

impl Solution {
    pub fn equal_frequency(word: String) -> bool {
        let mut char_count: HashMap<char, i32> = HashMap::new();
        for c in word.chars() {
            let e = char_count.entry(c).or_insert(0);
            *e += 1;
        }

        let mut value_count: BTreeMap<i32, i32> = BTreeMap::new();
        for v in char_count.values().cloned() {
            let e = value_count.entry(v).or_insert(0);
            *e += 1;
        }
        match value_count.len() {
            1 => {
                if char_count.len() == 2 {
                    return value_count.contains_key(&1);
                }
                true
            }
            2 => {
                if let Some(count) = value_count.get(&1) {
                    if count == &1 {
                        return true;
                    }
                }

                let keys: Vec<i32> = value_count.keys().cloned().collect();
                let values: Vec<i32> = value_count.values().cloned().collect();
                return keys[1] - keys[0] == 1 && (values[0] == 1 || values[1] == 1);
            }
            _ => false,
        }
    }
}
