impl Solution {
    pub fn valid_mountain_array(a: Vec<i32>) -> bool {
        if a.len() < 3 {
            return false;
        }

        let mut i: usize = 1;
        while i < a.len() {
            if a[i] <= a[i - 1] {
                break;
            }
            i += 1;
        }

        if i == a.len() || i == 1 {
            return false;
        }

        while i < a.len() {
            if a[i] >= a[i - 1] {
                return false;
            }
            i += 1;
        }

        true
    }
}
