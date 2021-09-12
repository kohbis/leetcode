impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let nums: HashSet<&i32> = arr.iter().collect();

        nums.into_iter()
            .map(|num| {
                if *num == arr.iter().filter(|&x| x == num).count() as i32 {
                    *num as i32
                } else {
                    -1 as i32
                }
            })
            .collect::<Vec<i32>>()
            .into_iter()
            .max()
            .unwrap()
    }
}
