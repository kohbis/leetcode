impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut res = 0;
        let mut cnt = 0;
        for c in s.chars() {
            cnt += if c == 'L' { 1 } else { -1 };
            if cnt == 0 {
                res += 1;
            }
        }
        res
    }
}
