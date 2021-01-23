impl Solution {
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        use std::cmp::{max, min};

        let len = grid.len();
        let mut row: Vec<i32> = vec![0; grid.len()];
        let mut col: Vec<i32> = vec![0; grid.len()];

        for i in 0..len {
            for j in 0..len {
                row[i] = max(row[i], grid[i][j]);
                col[j] = max(col[j], grid[i][j]);
            }
        }

        let mut res: i32 = 0;
        for i in 0..len {
            for j in 0..len {
                res += min(row[i], col[j]) - grid[i][j]
            }
        }

        res
    }
}
