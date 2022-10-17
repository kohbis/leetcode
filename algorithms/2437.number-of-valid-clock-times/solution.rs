impl Solution {
    pub fn count_time(time: String) -> i32 {
        let mut res = 1i32;
        let bs = time.as_bytes();

        res *= match (bs[0], bs[1]) {
            (b'?', b'?') => 24,
            (b'?', b'0'..=b'3') => 3,
            (b'?', b'4'..=b'9') => 2,
            (b'0'..=b'1', b'?') => 10,
            (b'2', b'?') => 4,
            _ => 1,
        };
        res *= if bs[3] == b'?' { 6 } else { 1 };
        res *= if bs[4] == b'?' { 10 } else { 1 };

        res
    }
}
