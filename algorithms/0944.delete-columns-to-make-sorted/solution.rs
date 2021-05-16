impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let mut count = 0;
        for col in 0..strs[0].len() {
            for row in 1..strs.len() {
                if strs[row - 1].chars().nth(col) > strs[row].chars().nth(col) {
                    count += 1;
                    break;
                }
            }
        }
        count
    }
}
