impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];

        let mut row_start = 0;
        let mut row_end = matrix.len() - 1;
        let mut col_start = 0;
        let mut col_end = matrix[0].len() - 1;

        let mut count = 0;
        let mut max = matrix.len() * matrix[0].len();

        while row_start <= row_end && col_start <= col_end {
            // →
            for i in col_start..=col_end {
                res.push(matrix[row_start][i]);
                count += 1;
            }
            row_start += 1;
            if count >= max {
                break;
            }

            // ↓
            for i in row_start..=row_end {
                res.push(matrix[i][col_end]);
                count += 1;
            }
            col_end -= 1;
            if count >= max {
                break;
            }

            // ←
            if row_start <= row_end {
                for i in (col_start..=col_end).rev() {
                    res.push(matrix[row_end][i]);
                    count += 1;
                }
                row_end -= 1;
            }
            if count >= max {
                break;
            }

            // ↑
            if col_start <= col_end {
                for i in (row_start..=row_end).rev() {
                    res.push(matrix[i][col_start]);
                    count += 1;
                }
                col_start += 1;
            }
        }

        res
    }
}
