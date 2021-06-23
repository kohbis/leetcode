impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let mut subset: Vec<i32> = vec![0];
        for n in nums {
            let curr_subset = subset.clone();
            for m in curr_subset {
                subset.push(n ^ m);
            }
        }
        subset.iter().sum()
    }
}
