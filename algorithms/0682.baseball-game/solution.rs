impl Solution {
    pub fn cal_points(ops: Vec<String>) -> i32 {
        let mut scores: Vec<i32> = vec![];

        for op in ops {
            match op.as_str() {
                "C" => {
                    scores.pop();
                }
                "D" => {
                    scores.push(scores[scores.len() - 1] * 2);
                }
                "+" => {
                    scores.push(scores[scores.len() - 2] + scores[scores.len() - 1]);
                }
                _ => {
                    scores.push(op.parse::<i32>().unwrap());
                }
            }
        }

        scores.iter().sum()
    }
}
