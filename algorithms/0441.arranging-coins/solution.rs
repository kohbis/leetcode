impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        let mut n = n;
        let mut res = 1i32;

        while 0 < n {
            res += 1;
            n -= res;
        }

        res - 1
    }
}
