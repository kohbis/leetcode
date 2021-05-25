impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![vec![0; matrix.len()]; matrix[0].len()];
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                res[j][i] = matrix[i][j]
            }
        }
        res
    }
}
