use std::collections::HashMap;

impl Solution {
    pub fn custom_sort_string(order: String, str: String) -> String {
        let mut counts: HashMap<char, usize> = HashMap::new();
        for c in str.chars() {
            let count = counts.entry(c).or_insert(0);
            *count += 1;
        }

        let mut res = String::from("");

        for c in order.chars() {
            if let Some(&n) = counts.get(&c) {
                let s = c.to_string().repeat(n);
                res.push_str(&s);
            }
            counts.remove(&c);
        }

        for (c, n) in counts {
            let s = c.to_string().repeat(n);
            res.push_str(&s);
        }

        res
    }
}
