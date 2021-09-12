impl Solution {
    pub fn can_make_arithmetic_progression(arr: Vec<i32>) -> bool {
        let mut reorder = arr.clone();
        reorder.sort();

        // using HashSet //
        // use std::collections::HashSet;
        // (0..(arr.len() - 1)).map(|i| reorder[i] - reorder[i + 1]).collect::<HashSet<i32>>().len() == 1

        // calc each conbinations //
        let diff = reorder[0] - reorder[1];
        for i in 1..(arr.len() - 1) {
            if diff != reorder[i] - reorder[i + 1] {
                return false;
            }
        }

        true
    }
}
