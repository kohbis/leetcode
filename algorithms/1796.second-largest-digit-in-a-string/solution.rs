impl Solution {
    pub fn second_highest(s: String) -> i32 {
        let digits = "0123456789";
        let mut nums: Vec<i32> = vec![];
        for c in s.chars() {
            if digits.contains(c) {
                nums.push(c as i32 - 48);
            }
        }
        nums.sort_unstable();
        nums.dedup();

        if nums.len() > 1 {
            nums[nums.len() - 2]
        } else {
            -1
        }
    }
}
