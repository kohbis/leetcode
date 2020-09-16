impl Solution {
    pub fn lucky_numbers (matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let row_len = matrix[0].len();

        let mut row_mins: Vec<i32> = Vec::new();
        let mut col_nums: Vec<Vec<i32>> = vec![vec![]; row_len];

        for row in matrix {
            for i in 0..row_len {
                col_nums[i].push(row[i]);
            }

            let row_min = row.iter().min().unwrap();
            row_mins.push(*row_min);
        }

        for col in col_nums {
            let col_max = col.iter().max().unwrap();
            if row_mins.contains(col_max) {
                return vec![*col_max];
            }
        }

        [].to_vec() // unreachable!()
    }
}
