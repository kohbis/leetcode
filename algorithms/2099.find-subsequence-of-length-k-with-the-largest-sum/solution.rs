impl Solution {
    pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let kusize = k as usize;
        let mut res: Vec<i32> = vec![];

        let mut target = nums.clone();
        target.sort_unstable();
        target.drain(..(nums.len() - kusize));

        for n in nums {
            if let Some(idx) = target.iter().position(|x| x == &n) {
                res.push(n);
                target.remove(idx);

                if target.len() == 0 {
                    break;
                }
            }
        }

        res
    }
}
