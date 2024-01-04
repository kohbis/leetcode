impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let rows = bank
            .iter()
            .map(|floor| floor.matches('1').count() as i32)
            .filter(|&count| count > 0)
            .collect::<Vec<_>>();
        rows.windows(2).fold(0, |acc, nums| acc + nums[0] * nums[1])
    }
}
