impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        for i in 1..=n {
            if Self::no_zero(i) && Self::no_zero(n - i) {
                return vec![i, n - i]
            }
        }

        unreachable!()
    }

    fn no_zero(num: i32) -> bool {
        let mut num = num;
        while num > 0 {
            if num % 10 == 0 {
                return false
            }

            num /= 10
        }

        true
    }
}
