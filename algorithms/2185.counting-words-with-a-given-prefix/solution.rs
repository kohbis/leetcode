impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        let mut count = 0i32;
        let p_len = pref.len();

        for word in words {
            if word.len() >= p_len && &word[..p_len] == pref {
                count += 1;
            }
        }

        count
    }
}
