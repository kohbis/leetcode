impl Solution {
    pub fn convert_to_title(n: i32) -> String {
        let mut res = String::from("");
        let mut n = n;
        let a = b'A' as u8;

        while n > 0 {
            n -= 1;
            res.insert(0, (a + ((n % 26) as u8)) as char);
            n /= 26;
        }

        res
    }
}
