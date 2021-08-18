impl Solution {
    pub fn max_score(s: String) -> i32 {
        let len = s.len();
        let bits: Vec<char> = s.chars().collect();
        let mut scores = vec![0; len - 1];

        let mut count_zero = 0;
        for left in 0..len - 1 {
            if bits[left] == '0' {
                count_zero += 1;
            }
            scores[left] += count_zero;
        }

        let mut count_one = 0;
        for right in (0..len - 1).rev() {
            if bits[right + 1] == '1' {
                count_one += 1;
            }
            scores[right] += count_one;
        }

        *scores.iter().max().unwrap() as i32
    }
}
