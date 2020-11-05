impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut i = 0;

        while i < arr.len() {
            let mut j = i.clone();
            while j < arr.len() {
                count += (i..=j).fold(0, |sum, idx| sum + arr[idx]);
                j += 2;
            }
            i += 1;
        }

        count
    }
}
