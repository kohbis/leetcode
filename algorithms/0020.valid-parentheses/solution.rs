use std::collections::HashMap;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = vec![];
        let brackets: HashMap<char, char> = HashMap::from([('(', ')'), ('{', '}'), ('[', ']')]);
        let close_brackets: Vec<char> = vec![')', '}', ']'];

        for c in s.chars() {
            if brackets.contains_key(&c) {
                stack.push(c);
            } else if close_brackets.contains(&c) {
                match stack.pop() {
                    Some(x) => {
                        if brackets[&x] != c {
                            return false;
                        }
                    }
                    _ => return false,
                }
            } else {
                return false;
            }
        }

        stack.is_empty()
    }
}
