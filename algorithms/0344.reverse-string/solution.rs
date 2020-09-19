impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        if s.len() > 0 {
            let mut i = 0;
            let mut j = s.len() - 1;

            while i < j {
                let buffer = s[i];
                s[i] = s[j];
                s[j] = buffer;
                i += 1;
                j -= 1;
            }
        }
    }
}
