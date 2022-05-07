impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut stack: Vec<i32> = Vec::new();
        let mut max = i32::min_value();

        for i in (0..nums.len()).rev() {
            if nums[i] < max {
                return true;
            }

            while !stack.is_empty() && *stack.last().unwrap() < nums[i] {
                max = stack.pop().unwrap();
            }

            stack.push(nums[i]);
        }

        false
    }
}
