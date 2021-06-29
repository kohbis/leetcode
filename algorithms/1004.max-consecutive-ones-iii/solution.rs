impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut longest = 0;
        let mut start = 0;

        for start in 0..nums.len() {
            if start > nums.len() - longest as usize {
                break;
            }
            let mut length = 0;
            let mut zeros = k;
            'sub: for n in start..nums.len() {
                if nums[n] == 0 {
                    zeros -= 1;
                    if zeros < 0 {
                        break 'sub;
                    }
                }
                length += 1;
            }

            if longest < length {
                longest = length;
            }
        }

        longest
    }
}
