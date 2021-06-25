impl Solution {
    pub fn hammingWeight(n: u32) -> i32 {
        // Lazy solution
        // n.count_ones() as i32

        let mut binary_string = n;
        let mut count = 0;
        for _ in 0..32 {
            count += (binary_string & 1) as i32;
            binary_string >>= 1;
        }
        count
    }
}
