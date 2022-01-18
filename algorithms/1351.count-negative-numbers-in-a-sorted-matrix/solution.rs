impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let mut res = 0i32;
        for g in grid {
            res += g.into_iter().filter(|&n| n < 0).count() as i32;
        }

        res
    }
}
