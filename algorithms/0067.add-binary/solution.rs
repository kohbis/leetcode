impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let v_a: Vec<char> = a.chars().collect();
        let v_b: Vec<char> = b.chars().collect();

        let (mut i_a, mut i_b, mut carry) = (a.len(), b.len(), 0);
        let mut res = String::new();

        while i_a > 0 || i_b > 0 || carry > 0 {
            if i_a > 0 {
                i_a -= 1;
                carry += v_a[i_a] as u8 - 48;
            }
            if i_b > 0 {
                i_b -= 1;
                carry += v_b[i_b] as u8 - 48;
            }

            res.insert(0, (carry % 2 + 48) as char);
            carry /= 2;
        }

        res
    }
}
