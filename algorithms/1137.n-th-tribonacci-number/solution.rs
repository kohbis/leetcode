impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        if n <= 1 {
            return n;
        }

        let mut tri = [0, 0, 1];
        for i in 2..n {
            tri.rotate_left(1);
            tri[2] = tri.iter().sum();
        }

        tri.iter().sum()
    }
}
