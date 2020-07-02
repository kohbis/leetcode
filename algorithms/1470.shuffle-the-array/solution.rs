impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut answer = Vec::with_capacity(nums.len());
        let n = n as usize;
        for i in 0..n {
            answer.push(nums[i]);
            answer.push(nums[i + n]);
        }
        answer
    }
}
