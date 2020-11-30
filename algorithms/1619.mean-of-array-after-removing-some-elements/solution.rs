impl Solution {
    pub fn trim_mean(arr: Vec<i32>) -> f64 {
        let mut arr = arr;
        arr.sort();

        let len = arr.len();
        let len_5percent: usize = len / 20;
        let len_90percent: usize = len * 9 / 10;

        let mut sum: i32 = 0;
        for i in len_5percent..(len_5percent+len_90percent) {
            sum += arr[i];
        }

        sum as f64 / len_90percent as f64
    }
}
