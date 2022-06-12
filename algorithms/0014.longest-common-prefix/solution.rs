impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let min_len = strs.iter().map(|s| s.len()).min().unwrap();
        for i in 0..min_len {
            let c = strs[0].chars().nth(i).unwrap();
            for s in &strs {
                if s.chars().nth(i).unwrap() != c {
                    return s[0..i].to_string();
                }
            }
        }

        strs[0][0..min_len].to_string()
    }
}
