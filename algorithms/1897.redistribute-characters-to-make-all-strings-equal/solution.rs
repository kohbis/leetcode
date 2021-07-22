impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let mut counts = vec![0; 26];
        let len = words.len();

        for word in words {
            for c in word.chars() {
                counts[c as usize - 'a' as usize] += 1;
            }
        }

        for count in counts {
            if count % len != 0 {
                return false;
            }
        }

        true
    }
}
