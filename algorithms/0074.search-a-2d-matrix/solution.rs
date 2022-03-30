impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let col_size = matrix[0].len();
        for row in matrix {
            if row[0] <= target && target <= row[col_size - 1] {
                return row.contains(&target);
            }
        }

        false
    }
}
