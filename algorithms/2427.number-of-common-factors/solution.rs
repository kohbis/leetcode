impl Solution {
    pub fn common_factors(a: i32, b: i32) -> i32 {
        let mut res: i32 = if a == b { 1 } else { 0 };
        for i in 1..=(a.max(b) / 2) {
            if a % i == 0 && b % i == 0 {
                res += 1;
            }
        }
        res
    }
}
