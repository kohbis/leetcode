use std::collections::HashMap;
use std::iter::FromIterator;

impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        let half = (arr.len() / 2) as i32;
        let mut hm: HashMap<i32, i32> = HashMap::new();
        for n in arr {
            let count = hm.entry(n).or_insert(0);
            *count += 1
        }

        let mut counts = Vec::from_iter(hm.values());
        counts.sort_unstable();

        let mut set_size = 0;
        let mut sum = 0;
        while counts.len() > 0 {
            if let Some(v) = counts.pop() {
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
