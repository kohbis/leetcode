impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res = vec![];
        Solution::backtrace(n, n, "".to_string(), &mut res);
        res
    }

    fn backtrace(left: i32, right: i32, mut current: String, res: &mut Vec<String>) {
        if left == 0 && right == 0 {
            res.push(current);
            return;
        }

        if 0 < left {
            current.push('(');
            Solution::backtrace(left - 1, right, current.clone(), res);
            current.pop();
        }

        if left < right {
            current.push(')');
            Solution::backtrace(left, right - 1, current.clone(), res);
            current.pop();
        }
    }
}
