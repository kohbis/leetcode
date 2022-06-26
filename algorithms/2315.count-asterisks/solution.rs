impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
        let mut count = 0i32;
        let mut inside = true;

        for c in s.chars() {
            match c {
                '|' => {
                    inside = !inside;
                }
                '*' => {
                    if inside {
                        count += 1;
                    }
                }
                _ => {}
            }
        }

        count
    }
}
