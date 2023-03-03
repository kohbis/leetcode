impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if let Some(i) = haystack.find(&needle) {
            i as _
        } else {
            -1
        }
    }
}
