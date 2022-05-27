impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        let mut num = num;
        let mut step = 0i32;

        while num > 0 {
            if num % 2 == 0 {
                num /= 2;
            } else {
                num -= 1;
            }

            step += 1;
        }

        step
    }
}
