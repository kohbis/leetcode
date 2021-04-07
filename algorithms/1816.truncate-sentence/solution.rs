impl Solution {
    pub fn truncate_sentence(s: String, k: i32) -> String {
        s.split_whitespace().collect::<Vec<&str>>()[..k as usize].join(" ")
    }
}
