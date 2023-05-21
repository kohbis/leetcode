impl Solution {
    pub fn min_length(s: String) -> i32 {
        let mut stack: Vec<char> = vec![];
        for c in s.chars() {
            match c {
                'B' => {
                    if !stack.is_empty() && stack[stack.len() - 1] == 'A' {
                        stack.pop();
                    } else {
                        stack.push(c);
                    }
                }
                'D' => {
                    if !stack.is_empty() && stack[stack.len() - 1] == 'C' {
                        stack.pop();
                    } else {
                        stack.push(c);
                    }
                }
                _ => stack.push(c),
            }
        }
        stack.len() as _
    }
}
