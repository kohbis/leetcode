impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let mut sum: i32 = 0;
        let len = mat.len();
        let mut i: usize = len / 2;

        if mat.len() % 2 == 1 {
            sum += mat[i][i];
        }

        while i > 0 {
            i -= 1;
            sum += mat[i][i]
                + mat[i][len - i - 1]
                + mat[len - i - 1][i]
                + mat[len - i - 1][len - i - 1];
        }

        sum
    }
}
