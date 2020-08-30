impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut res: i32 = 0;
        let &max = nums.last().unwrap();

        if target > max {
            res = nums.len() as i32;
        } else {
            for (i, &x) in nums.iter().enumerate() {
                if x >= target {
                    res = i as i32;
                    break;
                }
            }
        }

        res
    }
}
