impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let mut res = String::new();

        let mut chars1 = num1.chars().collect::<Vec<_>>();
        let mut chars2 = num2.chars().collect::<Vec<_>>();

        let mut carry = 0u8;
        while chars1.len() > 0 || chars2.len() > 0 || carry > 0 {
            let mut sum = carry;

            if let Some(digit1) = chars1.pop() {
                sum += digit1 as u8 - b'0';
            }

            if let Some(digit2) = chars2.pop() {
                sum += digit2 as u8 - b'0';
            }

            res.insert(0, (sum % 10u8 + b'0') as char);
            carry = sum / 10;
        }

        res
    }
}
