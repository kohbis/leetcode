impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        for i in 1..=n {
            if Solution::range_sum(1, i) == Solution::range_sum(i, n) {
                return i;
            }
        }
        -1
    }

    fn range_sum(start: i32, end: i32) -> i32 {
        (start..=end).fold(0i32, |sum, x| sum + x)
    }
}
