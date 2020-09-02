impl Solution {
    pub fn odd_cells(n: i32, m: i32, indices: Vec<Vec<i32>>) -> i32 {
        let mut matrix: Vec<Vec<i32>> = vec![vec![0; m as usize]; n as usize];
        let mut count: i32 = 0;

        // apply the increment to all indices
        for ind in indices {
            let (ri, ci) = (ind[0] as usize, ind[1] as usize);

            // row
            for cell in matrix[ri].iter_mut() {
               *cell += 1;
            }
            // column
            for row in matrix.iter_mut() {
                row[ci] += 1;
            }
        }

        for row in matrix {
            count += row.iter().filter(|cell| *cell % 2 != 0).count() as i32;
        }

        count
    }
}

