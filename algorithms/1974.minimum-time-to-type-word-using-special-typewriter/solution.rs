use std::cmp::min;

impl Solution {
    pub fn min_time_to_type(word: String) -> i32 {
        let mut prev = 0i32;
        let mut time = 0i32;

        for c in word.chars() {
            let curr = (c as u8 - b'a') as i32;
            let diff = (curr - prev).abs();

            time += min(diff, 26 - diff) + 1;

            prev = curr;
        }

        time
    }
}
