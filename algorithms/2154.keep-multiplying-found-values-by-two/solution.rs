impl Solution {
    pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
        let mut target = original;

        loop {
            if nums.contains(&target) {
                target *= 2;
            } else {
                break;
            }
        }

        target
    }
}
