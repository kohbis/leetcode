impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let mut seconds = 0i32;
        let mut desires = tickets.clone();

        loop {
            for i in 0..desires.len() {
                if desires[i] > 0 {
                    seconds += 1;
                    desires[i] -= 1;

                    if k as usize == i && desires[i] == 0 {
                        return seconds;
                    }
                }
            }
        }

        seconds
    }
}
