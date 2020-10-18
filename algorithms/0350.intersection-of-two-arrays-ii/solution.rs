impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();

        let short_list: Vec<i32>;
        let mut long_list: Vec<i32>;

        if nums1.len() < nums2.len() {
            short_list = nums1;
            long_list = nums2;
        } else {
            short_list = nums2;
            long_list = nums1;
        }

        for n in short_list {
            if let Some(idx) = long_list.iter().position(|&x| x == n ) {
                res.push(n);
                long_list.remove(idx);
            }
        }

        res
    }
}
