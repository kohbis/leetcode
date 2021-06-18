impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let mut num = n;
        let mut zero_count = 0;

        while num >= 5 {
            num /= 5;
            zero_count += num;
        }

        zero_count
    }
}
