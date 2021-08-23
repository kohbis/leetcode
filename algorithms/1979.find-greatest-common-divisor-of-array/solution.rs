impl Solution {
    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        fn gcd(x: i32, y: i32) -> i32 {
            if y == 0 {
                return x;
            }

            gcd(y, x % y)
        }

        let mut min = 1001;
        let mut max = 0;
        for n in nums {
            if n < min {
                min = n;
            }

            if n > max {
                max = n;
            }
        }

        gcd(max, min)
    }
}
