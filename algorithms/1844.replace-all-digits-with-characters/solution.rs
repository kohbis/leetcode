impl Solution {
    pub fn replace_digits(s: String) -> String {
        let mut res: Vec<u8> = vec![];
        let mut pre = b'a';
        for (i, &v) in s.as_bytes().iter().enumerate() {
            res.push(if i % 2 == 0 {
                pre = v;
                v
            } else {
                pre + v - b'0'
            })
        }
        String::from_utf8(res).unwrap()
    }
}
