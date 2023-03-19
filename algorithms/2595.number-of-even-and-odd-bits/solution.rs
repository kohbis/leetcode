impl Solution {
    pub fn even_odd_bit(n: i32) -> Vec<i32> {
        let (mut even, mut odd) = (0, 0);
        let b: String = format!("{:b}", n);
        for (i, c) in b.chars().rev().enumerate() {
            if c == '1' {
                if i % 2 == 0 {
                    even += 1;
                } else {
                    odd += 1;
                }
            }
        }
        vec![even, odd]
    }
}
