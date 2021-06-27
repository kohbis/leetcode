impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        let mut sum = 0;

        for div in (1..=(num / 2)) {
            if num % div == 0 {
                sum += div;
            }
        }

        num == sum
    }
}
