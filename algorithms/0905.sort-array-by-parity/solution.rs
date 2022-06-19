impl Solution {
    pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];

        for n in nums {
            if n % 2 == 0 {
                res.insert(0, n);
            } else {
                res.push(n);
            }
        }

        res
    }
}
