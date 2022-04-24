use std::collections::HashMap;

struct UndergroundSystem {
    check_ins: HashMap<i32, (String, i32)>,
    travel_times: HashMap<(String, String), Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl UndergroundSystem {
    fn new() -> Self {
        UndergroundSystem {
            check_ins: HashMap::new(),
            travel_times: HashMap::new(),
        }
    }

    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.check_ins.insert(id, (station_name, t));
    }

    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        if let Some((start_station, check_in_time)) = self.check_ins.get(&id) {
            let times = self
                .travel_times
                .entry((start_station.to_string(), station_name))
                .or_insert(Vec::new());
            (*times).push(t - check_in_time);
        }
    }

    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        if let Some(times) = self.travel_times.get(&(start_station, end_station)) {
            times.iter().sum::<i32>() as f64 / times.len() as f64
        } else {
            -1f64 // unreachable!()
        }
    }
}

// /**
//  * Your UndergroundSystem object will be instantiated and called as such:
//  * let obj = UndergroundSystem::new();
//  * obj.check_in(id, stationName, t);
//  * obj.check_out(id, stationName, t);
//  * let ret_3: f64 = obj.get_average_time(startStation, endStation);
//  */
