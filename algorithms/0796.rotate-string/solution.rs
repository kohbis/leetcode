impl Solution {
    pub fn rotate_string(a: String, b: String) -> bool {
        if a == "" && b == "" {
            return true;
        }

        for i in 1..a.len() {
            if b == [&a[i..], &a[..i]].join("") {
                return true;
            }
        }

        false
    }
}
