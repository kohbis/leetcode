impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let mid = nums.len() / 2;
        let (mut pos, mut neg) = (vec![], vec![]);
        for n in nums {
            if n > 0 {
                pos.push(n);
            } else {
                neg.push(n);
            }
        }

        let mut ans = vec![];
        for i in 0..mid {
            ans.push(pos[i]);
            ans.push(neg[i]);
        }

        ans
    }
}
