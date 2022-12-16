use std::collections::HashMap;

impl Solution {
    pub fn custom_sort_string(order: String, str: String) -> String {
        let mut count: HashMap<char, usize> = HashMap::new();
        for c in str.chars() {
            let entry = count.entry(c).or_insert(0);
            *entry += 1;
        }

        let mut res = String::from("");

        for c in order.chars() {
            if let Some(&n) = count.get(&c) {
                let s = c.to_string().repeat(n);
                res.push_str(&s);
            }
            count.remove(&c);
        }

        for (c, n) in count {
            let s = c.to_string().repeat(n);
            res.push_str(&s);
        }

        res
    }
}
