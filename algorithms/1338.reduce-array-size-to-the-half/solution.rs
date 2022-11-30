use std::collections::HashMap;
use std::iter::FromIterator;

impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        let half = (arr.len() / 2) as i32;
        let mut hm: HashMap<i32, i32> = HashMap::new();
        for n in arr {
            let entry = hm.entry(n).or_insert(0);
            *entry += 1
        }

        let mut count = Vec::from_iter(hm.values());
        count.sort_unstable();

        let mut set_size = 0;
        let mut sum = 0;
        while count.len() > 0 {
            if let Some(v) = count.pop() {
                sum += v;
                set_size += 1;

                if half <= sum {
                    return set_size;
                }
            }
        }

        set_size // unreachable!()
    }
}
