impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut res: i32 = 0;
        let mut target: Vec<i32> = heights.clone();
        target.sort();

        for (idx, &num) in heights.iter().enumerate() {
            if num != target[idx] {
                res += 1;
            }
        }

        res
    }
}
