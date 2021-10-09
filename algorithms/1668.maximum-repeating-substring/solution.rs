impl Solution {
    pub fn max_repeating(sequence: String, word: String) -> i32 {
        let mut k = &sequence.len() / &word.len();
        while sequence.find(&word.repeat(k)).is_none() {
            k -= 1;
        }

        k as _
    }
}
