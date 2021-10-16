impl Solution {
    pub fn minimum_moves(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();

        let mut count = 0;
        let mut i = 0;
        while s.len() > i {
            if chars[i] == 'X' {
                count += 1;
                i += 2;
            }

            i += 1;
        }

        count as _
    }
}
