impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let pairs: Vec<(i32, &str)> = vec![
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];

        let mut res = String::new();
        let mut num = num;
        for pair in pairs {
            for _ in 0..(num / pair.0) {
                res.push_str(pair.1);
            }

            num = num % pair.0;
        }

        res
    }
}
