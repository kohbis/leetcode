impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();

        Self::helper(n, k, 1, &mut Vec::new(), &mut res);

        res
    }

    fn helper(n: i32, k: i32, index: i32, current: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if k as usize == current.len() {
            res.push(current.to_vec());
        } else {
            for v in index..=n {
                current.push(v);
                Self::helper(n, k, v + 1, current, res);
                current.pop();
            }
        }
    }
}
