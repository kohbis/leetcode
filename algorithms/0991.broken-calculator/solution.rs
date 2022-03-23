impl Solution {
    pub fn broken_calc(start_value: i32, target: i32) -> i32 {
        let mut count = 0i32;
        let mut target = target;

        while start_value < target {
            count += 1;

            if target % 2 == 0 {
                target /= 2;
            } else {
                target += 1;
            }
        }

        count + start_value - target
    }
}
