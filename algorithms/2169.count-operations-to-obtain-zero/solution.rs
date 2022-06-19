impl Solution {
    pub fn count_operations(num1: i32, num2: i32) -> i32 {
        let (mut num1, mut num2) = (num1, num2);
        let mut res = 0i32;

        while num1 > 0 && num2 > 0 {
            if num1 < num2 {
                num2 -= num1;
            } else {
                num1 -= num2;
            }

            res += 1;
        }

        res
    }
}
