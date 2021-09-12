impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut count = 0;
        let allowed_chars: Vec<char> = allowed.chars().collect();

        for word in words {
            let mut only_allowed_chars = true;
            let word_chars: Vec<char> = word.chars().collect();

            for c in word_chars {
                if !allowed_chars.contains(&c) {
                    only_allowed_chars = false;
                    break;
                }
            }

            if only_allowed_chars {
                count += 1
            }
        }

        count
    }
}
