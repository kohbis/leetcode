use std::collections::HashMap;

impl Solution {
    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        fn gcd(x: i32, y: i32) -> i32 {
            if y == 0 {
                x
            } else {
                gcd(y, x % y)
            }
        }

        let mut map: HashMap<i32, i32> = HashMap::new();
        for d in deck {
            let entry = map.entry(d).or_insert(0);
            *entry += 1;
        }

        let values = map.values().cloned().collect::<Vec<i32>>();
        let mut x = values[0];
        for v in values {
            x = gcd(x, v);
        }

        x >= 2
    }
}
