impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        n > 0 && n & (n - 1) == 0 && n.trailing_zeros() & 1 == 0
    }
}
