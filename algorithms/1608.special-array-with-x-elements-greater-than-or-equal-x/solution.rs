impl Solution {
    pub fn special_array(nums: Vec<i32>) -> i32 {
        for i in 0..nums.len() {
            let x = (i + 1) as i32;
            let greater_count = nums.iter().filter(|&n| *n >= x).count() as i32;
            if greater_count == x {
                return x;
            }
        }

        -1
    }
}
