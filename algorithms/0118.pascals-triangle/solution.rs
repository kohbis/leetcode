impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();

        for i in 0..num_rows as usize {
            // init row
            res.push(vec![1; i + 1]);

            // calculation
            for j in 1..i {
                res[i][j] = res[i - 1][j - 1] + res[i - 1][j];
            }
        }

        res
    }
}
