impl Solution {
    pub fn count_prefixes(words: Vec<String>, s: String) -> i32 {
        words.iter().filter(|&p| s.starts_with(p)).count() as _
    }
}
