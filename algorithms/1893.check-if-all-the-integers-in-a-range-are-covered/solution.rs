impl Solution {
    pub fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
        let mut covered = vec![false; 51];

        for range in ranges {
            for i in range[0]..=range[1] {
                covered[i as usize] = true;
            }
        }

        for i in left..=right {
            if !covered[i as usize] {
                return false;
            }
        }

        true
    }
}
