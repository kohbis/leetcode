impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        if n == 1 {
            return vec![vec![1]];
        }

        let mut matrix = vec![vec![0; n as usize]; n as usize];

        let mut left = 0;
        let mut right = n as usize - 1;
        let mut top = 0;
        let mut bottom = n as usize - 1;

        let mut num = 0i32;
        while left <= right && top <= bottom {
            for i in left..=right {
                num += 1;
                matrix[top][i] = num;
            }
            top += 1;

            for i in top..=bottom {
                num += 1;
                matrix[i][right] = num;
            }
            right -= 1;

            for i in (left..=right).rev() {
                num += 1;
                matrix[bottom][i] = num;
            }
            bottom -= 1;

            for i in (top..=bottom).rev() {
                num += 1;
                matrix[i][left] = num;
            }
            left += 1;
        }

        matrix
    }
}
