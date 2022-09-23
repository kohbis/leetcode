impl Solution {
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        let mut count: Vec<i32> = vec![0; 10];
        for d in digits {
            count[d as usize] += 1;
        }

        let mut res: Vec<i32> = Vec::new();
        for n in (100..1000).step_by(2) {
            let (one, ten, hundred) = (n % 10, n / 10 % 10, n / 100);

            count[one] -= 1;
            count[ten] -= 1;
            count[hundred] -= 1;

            if count[one] >= 0 && count[ten] >= 0 && count[hundred] >= 0 {
                res.push(n as i32);
            }

            count[one] += 1;
            count[ten] += 1;
            count[hundred] += 1;
        }

        res
    }
}
