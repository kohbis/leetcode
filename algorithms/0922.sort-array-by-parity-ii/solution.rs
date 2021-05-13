impl Solution {
    pub fn sort_array_by_parity_ii(nums: Vec<i32>) -> Vec<i32> {
        let mut odds: Vec<i32> = vec![];
        let mut evens: Vec<i32> = vec![];
        for n in nums {
            if n % 2 == 0 {
                evens.push(n);
            } else {
                odds.push(n);
            }
        }

        let mut res: Vec<i32> = vec![];
        while !odds.is_empty() || !evens.is_empty() {
            res.push(evens.pop().unwrap());
            res.push(odds.pop().unwrap());
        }
        res
    }
}
