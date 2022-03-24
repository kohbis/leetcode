impl Solution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let mut indexes: Vec<usize> = vec![];
        for (i, c) in boxes.chars().enumerate() {
            if c == '1' {
                indexes.push(i);
            }
        }

        let mut ans: Vec<i32> = vec![0; boxes.len()];
        for i in 0..boxes.len() {
            for j in &indexes {
                ans[i] += ((j - i) as i32).abs();
            }
        }

        ans
    }
}
