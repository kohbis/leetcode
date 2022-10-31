impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        for m in 1..matrix.len() {
            for n in 1..matrix[0].len() {
                if matrix[m][n] != matrix[m - 1][n - 1] {
                    return false;
                }
            }
        }

        true
    }
}
