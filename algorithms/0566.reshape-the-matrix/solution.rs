impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        if mat[0].len() * mat.len() != (r * c) as usize {
            return mat;
        }

        let r = r as usize;
        let c = c as usize;
        let mut reshaped: Vec<Vec<i32>> = vec![vec![0; c]; r];

        let data: Vec<i32> = mat.into_iter().flatten().collect();
        for i in 0..r {
            for j in 0..c {
                reshaped[i][j] = data[i * c + j];
            }
        }

        reshaped
    }
}
