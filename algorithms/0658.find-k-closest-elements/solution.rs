impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let k_usize = k as usize;
        let mut left = 0;
        let mut right = arr.len() - 1;

        while (right - left + 1 > k_usize) {
            if arr[right] - x < x - arr[left] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        arr[left..(left + k_usize)].to_vec()
    }
}
