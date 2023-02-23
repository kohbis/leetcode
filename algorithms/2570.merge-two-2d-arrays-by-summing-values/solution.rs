use std::collections::BTreeMap;

impl Solution {
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut bm: BTreeMap<i32, i32> = BTreeMap::new();
        for x in nums1.iter().chain(nums2.iter()) {
            *bm.entry(x[0]).or_default() += x[1];
        }
        bm.into_iter().map(|(k, v)| vec![k, v]).collect()
    }
}
