impl Solution {
    pub fn convert_to_base7(num: i32) -> String {
        if num == 0 {
            return "0".to_string();
        }

        let mut res: String = "".to_string();
        let mut n: i32 = num.abs();

        while n > 0 {
            let rem = n % 7;
            n /= 7;
            res.insert_str(0, &rem.to_string())
        }

        if num < 0 {
            res.insert_str(0, "-")
        }
        res
    }
}
