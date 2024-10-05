impl Solution {
    pub fn min_element(nums: Vec<i32>) -> i32 {
        let mut sums = vec![];
        for num in nums {
            let digits = num
                .to_string()
                .chars()
                .map(|d| d.to_digit(10).unwrap() as i32)
                .collect::<Vec<_>>();
            let sum = digits.iter().sum();
            sums.push(sum);
        }
        return *sums.iter().min().unwrap();
    }
}
