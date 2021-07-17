impl Solution {
    pub fn count_triples(n: i32) -> i32 {
        let mut count = 0 as i32;

        for a in 1..=n {
            for b in a..=n {
                for c in b..=n {
                    if a * a + b * b == c * c {
                        count += 2;
                    }
                }
            }
        }

        count
    }
}
