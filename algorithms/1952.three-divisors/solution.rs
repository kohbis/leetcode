impl Solution {
    pub fn is_three(n: i32) -> bool {
        let mut divs = 2;

        for i in 2..n {
            if n % i == 0 {
                divs += 1;
            }

            if divs > 3 {
                return false;
            }
        }

        divs == 3
    }
}
