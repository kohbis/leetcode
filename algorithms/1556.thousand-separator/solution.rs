impl Solution {
    pub fn thousand_separator(n: i32) -> String {
        let mut res = String::new();
        let s_n = n.to_string();
        for (idx, c) in s_n.chars().rev().enumerate() {
            res.insert(0, c);
            if (idx + 1) % 3 == 0 && (idx + 1) != s_n.len() {
                res.insert(0, '.');
            }
        }

        res
    }
}
