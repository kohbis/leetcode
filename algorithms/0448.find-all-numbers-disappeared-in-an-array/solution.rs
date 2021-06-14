impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut appeared = vec![false; nums.len()];
        for n in nums {
            appeared[n as usize - 1] = true;
        }

        appeared
            .iter()
            .enumerate()
            .filter_map(|(i, v)| if *v { None } else { Some(i as i32 + 1) })
            .collect::<Vec<i32>>()
    }
}
