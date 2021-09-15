impl Solution {
    pub fn count_quadruplets(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut count = 0;

        for a in 0..(len - 3) {
            for b in (a + 1)..(len - 2) {
                for c in (b + 1)..(len - 1) {
                    for d in (c + 1)..len {
                        if nums[a] + nums[b] + nums[c] == nums[d] {
                            count += 1;
                        }
                    }
                }
            }
        }

        count
    }
}
