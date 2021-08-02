impl Solution {
    pub fn count_good_substrings(s: String) -> i32 {
        let mut count = 0;

        let c = s.chars().collect::<Vec<char>>();
        for i in 2..s.len() {
            if c[i - 2] != c[i - 1] && c[i - 1] != c[i] && c[i] != c[i - 2] {
                count += 1;
            }
        }

        count
    }
}
