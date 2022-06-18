impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let steps = n as usize;
        let mut a: Vec<i32> = vec![0; steps];
        for i in 0..steps {
            a[i] = match i {
                0 => 1,
                1 => 2,
                _ => a[i - 2] + a[i - 1],
            }
        }
        a[steps - 1]
    }
}
