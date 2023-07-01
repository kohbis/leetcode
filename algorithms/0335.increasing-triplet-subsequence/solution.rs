impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let (mut i, mut j) = (i32::MAX, i32::MAX);
        for n in nums {
            if n < i {
                i = n;
                continue;
            }
            if i < n && n < j {
                j = n;
                continue;
            }
            if j < n {
                return true;
            }
        }
        false
    }
}
