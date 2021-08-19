impl Solution {
    pub fn check_record(s: String) -> bool {
        let mut a = 0;
        let mut l = 0;

        for c in s.chars() {
            match c {
                'A' => {
                    a += 1;
                    l = 0;
                }
                'L' => l += 1,
                _ => l = 0,
            }

            if a >= 2 || l >= 3 {
                return false;
            }
        }

        true
    }
}
