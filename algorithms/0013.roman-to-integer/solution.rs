use std::collections::HashMap;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut hm: HashMap<&str, i32> = HashMap::from([
            ("I", 1),
            ("V", 5),
            ("X", 10),
            ("L", 50),
            ("C", 100),
            ("D", 500),
            ("M", 1000),
            ("IV", 4),
            ("IX", 9),
            ("XL", 40),
            ("XC", 90),
            ("CD", 400),
            ("CM", 900),
        ]);

        let mut res: i32 = 0;
        let mut i: usize = 0;
        while i < s.len() {
            let mut key = if i < s.len() - 1 && hm.contains_key(&s[i..=i + 1]) {
                &s[i..=i + 1]
            } else {
                &s[i..i + 1]
            };

            res += &hm[key];
            i += key.len();
        }

        res
    }
}
