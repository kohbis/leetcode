use std::collections::HashMap;

impl Solution {
    pub fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
        let mut map: HashMap<String, String> = HashMap::new();
        for k in knowledge {
            map.insert(k[0].clone(), k[1].clone());
        }

        let mut res: Vec<String> = vec![];

        let chars: Vec<char> = s.chars().collect();
        let mut tmp = "".to_string();
        for c in chars {
            match c {
                '(' => {
                    if tmp.len() > 0 {
                        res.push(tmp.clone());
                    }
                    tmp = "".to_string();
                }
                ')' => {
                    if let Some(value) = map.get(&tmp) {
                        res.push(value.clone());
                    } else {
                        res.push("?".to_string());
                    }
                    tmp = "".to_string();
                }
                _ => {
                    tmp.push(c);
                }
            }
        }

        if tmp.len() > 0 {
            res.push(tmp);
        }

        res.join("")
    }
}
