impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut horiz = grid.clone().into_iter().flatten().collect::<Vec<i32>>();
        let time = k as usize % horiz.len();
        for _ in 0..time {
            let tmp = horiz.pop().unwrap();
            horiz.insert(0, tmp);
        }

        horiz
            .chunks(grid[0].len())
            .into_iter()
            .map(|chunk| chunk.to_vec())
            .collect()
    }
}
