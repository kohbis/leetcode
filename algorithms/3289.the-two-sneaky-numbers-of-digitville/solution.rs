use std::collections::HashMap;

impl Solution {
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut sneaky_numbers = Vec::new();
        let mut count = HashMap::new();
        for num in nums {
            *count.entry(num).or_insert(0) += 1;
            if count[&num] > 1 {
                sneaky_numbers.push(num);
            }
            if sneaky_numbers.len() == 2 {
                break;
            }
        }
        sneaky_numbers
}
