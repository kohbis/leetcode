impl Solution {
    pub fn get_smallest_string(s: String) -> String {
        let mut s = s.as_bytes().to_vec();
        for i in 0..s.len() - 1 {
            if s[i] % 2 == s[i + 1] % 2 {
                if s[i] > s[i + 1] {
                    s.swap(i, i + 1);
                    break;
                }
            }
        }
        String::from_utf8(s).unwrap()
    }
}
