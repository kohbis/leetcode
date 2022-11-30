impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let mut count = vec![0; 26];
        let len = words.len();

        for word in words {
            for c in word.chars() {
                count[c as usize - 'a' as usize] += 1;
            }
        }

        for x in count {
            if x % len != 0 {
                return false;
            }
        }

        true
    }
}
