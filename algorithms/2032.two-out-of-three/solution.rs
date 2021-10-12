use std::collections::HashMap;

impl Solution {
    pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
        let mut res = vec![];

        for n in 1..=100 {
            let mut count = 0u32;

            if nums1.contains(&n) {
                count += 1;
            }

            if nums2.contains(&n) {
                count += 1;
            }

            if nums3.contains(&n) {
                count += 1;
            }

            if count >= 2 {
                res.push(n as i32);
            }
        }

        res
    }
}
