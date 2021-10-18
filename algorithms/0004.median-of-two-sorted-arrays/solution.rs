impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let nums = Self::merge(&nums1, &nums2);
        let mut mid = nums.len() / 2;

        if nums.len() % 2 == 0 {
            (nums[mid - 1] + nums[mid]) as f64 / 2.0
        } else {
            nums[mid] as _
        }
    }

    fn merge(nums1: &[i32], nums2: &[i32]) -> Vec<i32> {
        let mut res = vec![];

        let mut i = 0;
        let mut j = 0;

        while i < nums1.len() && j < nums2.len() {
            if nums1[i] < nums2[j] {
                res.push(nums1[i]);
                i += 1;
            } else {
                res.push(nums2[j]);
                j += 1;
            }
        }

        while i < nums1.len() {
            res.push(nums1[i]);
            i += 1;
        }

        while j < nums2.len() {
            res.push(nums2[j]);
            j += 1;
        }

        res
    }
}
