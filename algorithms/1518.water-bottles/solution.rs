impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut count = 0;
        let mut full = num_bottles;
        let mut empty = 0;

        while 0 < full {
            count += full;
            empty += full;
            full = empty / num_exchange;
            empty = empty % num_exchange;
        }

        count
    }
}
