impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut res: Vec<i32> = vec![0];
        let range = 2_i32.pow(n as u32);
        for i in 1..range {
            res.push(i ^ (i >> 1));
        }
        res
    }
}
