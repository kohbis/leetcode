use std::cmp;

impl Solution {
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        let (mut xy, mut yz, mut xz) = (0, 0, 0);

        for i in 0..grid.len() {
            let (mut max_xz, mut max_yz) = (0, 0);

            for j in 0..grid.len() {
                if grid[i][j] > 0 {
                    xy += 1;
                }
                max_xz = cmp::max(max_xz, grid[i][j]);
                max_yz = cmp::max(max_yz, grid[j][i]);
            }

            xz += max_xz;
            yz += max_yz;
        }

        xy + xz + yz
    }
}
