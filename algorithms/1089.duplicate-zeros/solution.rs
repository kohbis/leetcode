impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut curr = 0;
        while arr.len() - 1 > curr {
            if arr[curr] == 0 {
                arr.insert(curr, 0);
                arr.pop();
                curr += 1;
            }
            curr += 1;
        }
    }
}
