impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        nums1.drain((m as usize)..(nums1.len() as usize));
        for num in nums2 {
            nums1.push(*num);
        }
        nums1.sort()
    }
}
