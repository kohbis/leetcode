impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let mut sums: Vec<i32> = vec![];
        let mut tmp: Vec<i32> = vec![];

        for n in nums {
            if tmp.len() > 0 {
                if tmp.last().unwrap() >= &n {
                    sums.push(tmp.iter().sum());
                    tmp = vec![]
                }
            }
            tmp.push(n)
        }
        sums.push(tmp.iter().sum());

        sums.into_iter().max().unwrap()
    }
}
