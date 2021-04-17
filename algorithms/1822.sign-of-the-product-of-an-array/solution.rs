impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        let mut negative_count: i32 = 0;
        for n in nums {
            if n == 0 {
                return 0
            }
            if 0 > n {
                negative_count += 1
            }
        }

        if negative_count % 2 == 0 {
            1
        } else {
            -1
        }
    }
}
