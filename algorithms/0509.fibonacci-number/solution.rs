impl Solution {
    pub fn fib(n: i32) -> i32 {
        let (mut a, mut b) = (0, 1);
        for _ in 0..n {
            let tmp = a + b;
            a = b;
            b = tmp;
        }
        a
    }
}
