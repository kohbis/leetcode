impl Solution {
    pub fn number_of_matches(n: i32) -> i32 {
        let mut n = n;
        let mut count: i32 = 0;

        while n > 1 {
            if n % 2 == 0 {
                n = n / 2;
                count += n;
            } else {
                n = (n - 1) / 2;
                count += n + 1;
            }
        }

        count
    }
}
