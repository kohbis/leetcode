impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut count: i32 = 0;
        let mut prev: i32 = 0;
        for num in nums {
            if prev >= num {
                count += (prev + 1) - num;
                prev += 1
            } else {
                prev = num
            }
        }
        count
    }
}
