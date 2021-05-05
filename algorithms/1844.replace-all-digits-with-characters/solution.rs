impl Solution {
    pub fn replace_digits(s: String) -> String {
        let mut res: Vec<u8> = vec![];
        let mut pre = b'a';
        for (i, &v) in s.as_bytes().iter().enumerate() {
            res.push(match i % 2 == 0 {
                true => {
                    pre = v;
                    v
                }
                false => pre + v - b'0',
            })
        }
        String::from_utf8(res).unwrap()
    }
}
