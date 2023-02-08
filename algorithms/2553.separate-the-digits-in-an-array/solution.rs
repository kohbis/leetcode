impl Solution {
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        for num in nums {
            Solution::push_digits(&mut res, num)
        }
        res
    }

    fn push_digits(res: &mut Vec<i32>, num: i32) {
        for c in num.to_string().chars() {
            if let Some(d) = c.to_digit(10) {
                res.push(d as i32);
            }
        }
    }
}
