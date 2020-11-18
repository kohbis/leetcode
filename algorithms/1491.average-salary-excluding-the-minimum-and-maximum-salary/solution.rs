impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let (mut min, mut max) = (salary[0], 0);
        let len = salary.len();
        let mut sum = 0;

        for s in salary {
            if s < min {
                min = s
            };
            if s > max {
                max = s
            };

            sum += s;
        }

        (sum - min - max) as f64 / (len - 2) as f64
    }
}
