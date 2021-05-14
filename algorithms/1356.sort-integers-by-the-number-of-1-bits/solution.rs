impl Solution {
    pub fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
        let mut arr = arr.clone();
        arr.sort_unstable_by_key(|i| (i.count_ones(), *i));
        arr
    }
}
