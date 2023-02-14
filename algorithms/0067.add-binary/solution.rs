impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let v: Vec<char> = a.chars().collect();
        let w: Vec<char> = b.chars().collect();

        let (mut i, mut j, mut carry) = (a.len(), b.len(), 0);
        let mut res = String::new();

        while i > 0 || j > 0 || carry > 0 {
            if i > 0 {
                i -= 1;
                carry += v[i] as u8 - b'0';
            }
            if j > 0 {
                j -= 1;
                carry += w[j] as u8 - b'0';
            }

            res.insert(0, (carry % 2 + b'0') as char);
            carry /= 2;
        }

        res
    }
}
