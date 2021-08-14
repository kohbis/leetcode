use std::cmp::min;

impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        if ops.len() == 0 {
            return m * n;
        }

        let mut min_m = ops[0][0];
        let mut min_n = ops[0][1];

        for i in 1..ops.len() {
            min_m = min(min_m, ops[i][0]);
            min_n = min(min_n, ops[i][1]);
        }

        min_m * min_n
    }
}
