impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut count: Vec<(usize, i32)> = mat
            .iter()
            .enumerate()
            .map(|(i, soldiers)| (i, soldiers.iter().sum()))
            .collect();
        count.sort_by(|a, b| a[1].cmp(&b[1]));
        count.iter().take(k as usize).map(|v| v[0] as i32).collect()
    }
}
