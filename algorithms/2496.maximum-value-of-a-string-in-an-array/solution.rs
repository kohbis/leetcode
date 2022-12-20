impl Solution {
    pub fn maximum_value(strs: Vec<String>) -> i32 {
        strs.iter().fold(0, |res, s| match s.parse::<i32>() {
            Ok(n) => res.max(n),
            Err(_) => res.max(s.len() as i32),
        })
    }
}
