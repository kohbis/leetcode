impl Solution {
    pub fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
        let keys: Vec<char> = keys_pressed.chars().collect();

        let mut slowest_time = release_times[0];
        let mut slowest_key = keys[0];

        for i in 1..release_times.len() {
            let time = release_times[i] - release_times[i - 1];

            if time > slowest_time {
                slowest_time = time;
                slowest_key = keys[i];
            } else if time == slowest_time && keys[i] > slowest_key {
                slowest_key = keys[i];
            }
        }

        slowest_key
    }
}
