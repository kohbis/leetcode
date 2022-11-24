impl Solution {
    pub fn unequal_triplets(nums: Vec<i32>) -> i32 {
        let mut res = 0i32;

        for i in 0..(nums.len() - 2) {
            for j in i + 1..(nums.len() - 1) {
                for k in j + 1..nums.len() {
                    if nums[i] != nums[j] && nums[j] != nums[k] && nums[k] != nums[i] {
                        res += 1;
                    }
                }
            }
        }

        res
    }
}
