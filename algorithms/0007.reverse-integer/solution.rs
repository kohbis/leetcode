impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut res: i64 = x as i64;

        // reverse
        res = res.abs().to_string().chars().rev().collect::<String>().parse().unwrap();

        // resign
        if x < 0 {
          res = res * -1
        }

        // check for i32 overflow
        if std::i32::MIN as i64 > res || res > std::i32::MAX as i64 {
            return 0;
        }

        res as i32
    }
}
