impl Solution {
    pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![];

        for n in nums {
            if n % 2 == 0 {
                ans.insert(0, n);
            } else {
                ans.push(n);
            }
        }

        ans
    }
}
