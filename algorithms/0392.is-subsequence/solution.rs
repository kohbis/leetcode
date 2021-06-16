impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut t_chars = t.chars();
        for c in s.chars() {
            match t_chars.find(|&x| x == c) {
                Some(_) => (),
                None => return false,
            }
        }
        true
    }
}
