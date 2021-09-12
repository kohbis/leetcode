impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut rows: Vec<Vec<i32>> = vec![vec![1]];

        for i in 1..=row_index as usize {
            // init row
            rows.push(vec![1; i + 1]);

            // calculation
            for j in 1..i {
                rows[i][j] = rows[i - 1][j - 1] + rows[i - 1][j];
            }
        }

        rows.last().unwrap().to_vec()
    }
}
