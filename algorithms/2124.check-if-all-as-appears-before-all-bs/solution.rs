impl Solution {
    pub fn check_string(s: String) -> bool {
        let mut appeared = false;
        for c in s.chars() {
            match c {
                'a' => {
                    if appeared {
                        return false;
                    }
                }
                'b' => appeared = true,
                _ => {}
            }
        }

        true
    }
}
