impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut max: i32 = 0;
        let mut pos: i32 = 0;

        for g in gain {
            pos += g;
            if pos > max {
                max = pos
            }
        }

        max
    }
}
