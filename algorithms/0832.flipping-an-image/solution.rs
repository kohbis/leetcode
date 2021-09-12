impl Solution {
    pub fn flip_and_invert_image(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        a.iter()
            .map(|line| line.iter().rev().map(|bit| bit ^ 1).collect())
            .collect()
    }
}
