impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];

        for n in left..=right {
            if n < 10 {
                res.push(n);
                continue;
            }

            let divisor = n
                .to_string()
                .chars()
                .map(|c| c as i32 - 48)
                .collect::<Vec<i32>>();

            let mut self_dividing = false;
            for d in divisor {
                if d != 0 && n % d == 0 {
                    self_dividing = true;
                } else {
                    self_dividing = false;
                    break;
                }
            }

            if self_dividing {
                res.push(n);
            }
        }

        res
    }
}
